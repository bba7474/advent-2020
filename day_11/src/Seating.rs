use std::str::FromStr;
use std::fmt;

pub struct Seating {
    seats: Vec<Vec<Seat>>
}

impl Seating {
    pub fn apply_rules(&mut self) {
        let mut to_occupy = Vec::new();
        let mut to_vacate = Vec::new();

        for row in 0..self.seats.len() {
            for col in 0..self.seats[row].len() {
                let adj_occupied = get_adjacent_coords(col, row, self.seats[row].len() - 1, self.seats.len() - 1).iter()
                    .map(|(x, y)| &self.seats[*y][*x])
                    .filter(|s| s.is_occupied())
                    .count();

                if self.seats[row][col].is_occupied() && adj_occupied >= 4 {
                    to_vacate.push((col, row));
                }
                if self.seats[row][col].is_vacant() && adj_occupied == 0 {
                    to_occupy.push((col, row));
                }
            }
        }

        to_occupy.iter().for_each(|(x, y)| self.seats[*y][*x].occupy());
        to_vacate.iter().for_each(|(x, y)| self.seats[*y][*x].vacate());
    }

    pub fn count_occupied(&self) -> i32 {
        self.seats.iter()
            .map(|row| row.iter().filter(|s| s.is_occupied()).count())
            .map(|u| u as i32)
            .sum()
    }
}

fn get_adjacent_coords(x: usize, y: usize, x_max: usize, y_max: usize) -> Vec<(usize, usize)> {
    let mut adj = Vec::new();
    if x != 0 {
        adj.push((x-1, y));
        if y != 0 {
            adj.push((x-1, y-1));
        }
        if y < y_max {
            adj.push((x-1, y+1));
        }
    }
    if x < x_max {
        adj.push((x+1, y));
        if y < y_max {
            adj.push((x+1, y+1));
        }
    }
    if y != 0 {
        adj.push((x, y-1));
        if x < x_max{
            adj.push((x+1, y-1));
        }
    }
    if y < y_max {
        adj.push((x, y+1));
    }

    adj
}

impl FromStr for Seating {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vec = Vec::new();

        for line in s.lines() {
            vec.push(line.chars().map(|c| c.to_string().parse::<Seat>().unwrap()).collect());
        }

        Ok(Seating{
            seats: vec
        })
    }
}

struct Seat {
    seat_type: String
}

impl Seat {
    pub fn is_occupied(&self) -> bool {
        self.seat_type == "#"
    }

    pub fn is_vacant(&self) -> bool {
        self.seat_type == "L"
    }

    pub fn occupy(&mut self) {
        self.seat_type = String::from("#");
    }

    pub fn vacate(&mut self) {
        self.seat_type = String::from("L");
    }
}

impl FromStr for Seat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Seat{
            seat_type: String::from(s)
        })
    }
}

impl fmt::Debug for Seat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.seat_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = ".LLL\n.L.#\n.L##";
        let seating = input.parse::<Seating>().unwrap();

        assert!(seating.seats[0][0].is_floor());
        assert!(seating.seats[0][1].is_vacant());
        assert!(seating.seats[0][2].is_vacant());
        assert!(seating.seats[0][3].is_vacant());
        assert!(seating.seats[1][0].is_floor());
        assert!(seating.seats[1][1].is_vacant());
        assert!(seating.seats[1][2].is_floor());
        assert!(seating.seats[1][3].is_occupied());
        assert!(seating.seats[2][0].is_floor());
        assert!(seating.seats[2][1].is_vacant());
        assert!(seating.seats[2][2].is_occupied());
        assert!(seating.seats[2][3].is_occupied());
    }

    #[test]
    fn test_rules() {
        let input = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";
        let mut seating = input.parse::<Seating>().unwrap();

        seating.apply_rules();
        assert_eq!(seating.count_occupied(), 71);

        seating.apply_rules();
        assert_eq!(seating.count_occupied(), 20);

        seating.apply_rules();
        seating.apply_rules();
        seating.apply_rules();
        assert_eq!(seating.count_occupied(), 37);
    }


}