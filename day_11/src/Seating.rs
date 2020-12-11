use std::str::FromStr;
use std::fmt;

pub struct Seating {
    seats: Vec<Vec<Seat>>,
    adj_seat_coords: Vec<Vec<Vec<(usize, usize)>>>,
    first_seat_visible_coords: Vec<Vec<Vec<(usize, usize)>>>
}

impl Seating {
    pub fn apply_rules_1(&mut self) {
        let mut to_occupy = Vec::new();
        let mut to_vacate = Vec::new();

        for row in 0..self.seats.len() {
            for col in 0..self.seats[row].len() {

                let adj_occupied = &self.adj_seat_coords[row][col].iter()
                    .map(|(x, y)| &self.seats[*y][*x])
                    .filter(|s| s.is_occupied())
                    .count();

                if self.seats[row][col].is_occupied() && *adj_occupied >= 4 {
                    to_vacate.push((col, row));
                }
                if self.seats[row][col].is_vacant() && *adj_occupied == 0 {
                    to_occupy.push((col, row));
                }
            }
        }

        to_occupy.iter().for_each(|(x, y)| self.seats[*y][*x].occupy());
        to_vacate.iter().for_each(|(x, y)| self.seats[*y][*x].vacate());
    }

    pub fn apply_rules_2(&mut self) {
        let mut to_occupy = Vec::new();
        let mut to_vacate = Vec::new();

        for row in 0..self.seats.len() {
            for col in 0..self.seats[row].len() {

                let count_first_seats_occupied = &self.first_seat_visible_coords[row][col].iter()
                    .map(|(x, y)| &self.seats[*y][*x])
                    .filter(|s| s.is_occupied())
                    .count();


                if self.seats[row][col].is_occupied() && *count_first_seats_occupied >= 5 {
                    to_vacate.push((col, row));
                }
                if self.seats[row][col].is_vacant() && *count_first_seats_occupied == 0 {
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

fn get_visible_coords_from(x: usize, y: usize, x_max: usize, y_max: usize, limit: Option<usize>) -> Vec<Vec<(usize, usize)>> {
    let n_coords = get_visible_coords_in_direction(x, y, x_max, y_max, |x| x, |y| y-1, limit);
    let ne_coords = get_visible_coords_in_direction(x, y, x_max, y_max, |x| x+1, |y| y-1, limit);
    let e_coords = get_visible_coords_in_direction(x, y, x_max, y_max, |x| x+1, |y| y, limit);
    let se_coords = get_visible_coords_in_direction(x, y, x_max, y_max, |x| x+1, |y| y+1, limit);
    let s_coords = get_visible_coords_in_direction(x, y, x_max, y_max, |x| x, |y| y+1, limit);
    let sw_coords = get_visible_coords_in_direction(x, y, x_max, y_max, |x| x-1, |y| y+1, limit);
    let w_coords = get_visible_coords_in_direction(x, y, x_max, y_max, |x| x-1, |y| y, limit);
    let nw_coords = get_visible_coords_in_direction(x, y, x_max, y_max, |x| x-1, |y| y-1, limit);

    vec![n_coords, ne_coords, e_coords, se_coords, s_coords, sw_coords, w_coords, nw_coords]
}

fn get_visible_coords_in_direction<F, G>(x: usize, y: usize, x_max: usize, y_max: usize, x_transform: F, y_transform: G, limit: Option<usize>) -> Vec<(usize, usize)> where F: Fn(i32) -> i32, G: Fn(i32) -> i32 {
    let mut coords = Vec::new();
    let mut x_coord = x_transform(x.clone() as i32);
    let mut y_coord = y_transform(y.clone() as i32);

    while x_coord >= 0 && x_coord <= x_max as i32 && y_coord >= 0 && y_coord <= y_max as i32 && (limit.is_none() || (coords.len() < limit.unwrap())) {
        coords.push((x_coord, y_coord));
        x_coord = x_transform(x_coord);
        y_coord = y_transform(y_coord);
    }

    coords.iter().map(|(x, y)| (*x as usize, *y as usize)).collect()
}

fn get_adjacent_seat_coords_from_coords(x: usize, y: usize, seats: &Vec<Vec<Seat>>) -> Vec<(usize, usize)> {
    let coords_vis_from = get_visible_coords_from(x, y, seats[y].len() - 1, seats.len() - 1, Some(1));

    let mut first_seat_coords = Vec::new();
    for direction_coords in coords_vis_from {
        for (x1, y1) in direction_coords {
            if !seats[y1][x1].is_floor() {
                first_seat_coords.push((x1, y1));
                break;
            }
        }
    }

    first_seat_coords
}

fn get_first_seat_visible_coords_from_coords(x: usize, y: usize, seats: &Vec<Vec<Seat>>) -> Vec<(usize, usize)> {
    let coords_vis_from = get_visible_coords_from(x, y, seats[y].len() - 1, seats.len() - 1, None);

    let mut first_seat_coords = Vec::new();
    for direction_coords in coords_vis_from {
        for (x1, y1) in direction_coords {
            if !seats[y1][x1].is_floor() {
                first_seat_coords.push((x1, y1));
                break;
            }
        }
    }

    first_seat_coords
}

impl FromStr for Seating {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut seats: Vec<Vec<Seat>> = Vec::new();

        for line in s.lines() {
            seats.push(line.chars().map(|c| c.to_string().parse::<Seat>().unwrap()).collect());
        }

        let mut adj_seat_coords = Vec::new();
        let mut first_seat_visible_coords = Vec::new();
        for (i, row) in seats.iter().enumerate() {
            let mut row_adj_coords = Vec::new();
            let mut row_first_vis_coords = Vec::new();
            for (j, _) in row.iter().enumerate() {
                row_adj_coords.push(get_adjacent_seat_coords_from_coords(j, i, &seats));
                row_first_vis_coords.push(get_first_seat_visible_coords_from_coords(j, i, &seats));
            }
            adj_seat_coords.push(row_adj_coords);
            first_seat_visible_coords.push(row_first_vis_coords);
        }

        Ok(Seating{
            seats,
            adj_seat_coords,
            first_seat_visible_coords
        })
    }
}

impl Clone for Seating {
    fn clone(&self) -> Self {
        return Seating {
            seats: self.seats.iter().map(|row| row.iter().map(|s| s.clone()).collect()).collect(),
            adj_seat_coords: self.adj_seat_coords.clone(),
            first_seat_visible_coords: self.first_seat_visible_coords.clone()
        };
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

    pub fn is_floor(&self) -> bool {
        self.seat_type == "."
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

impl Clone for Seat {
    fn clone(&self) -> Self {
        return Seat {
            seat_type: self.seat_type.clone()
        };
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
    fn test_rules_1() {
        let input = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";
        let mut seating = input.parse::<Seating>().unwrap();

        seating.apply_rules_1();
        assert_eq!(seating.count_occupied(), 71);

        seating.apply_rules_1();
        assert_eq!(seating.count_occupied(), 20);

        seating.apply_rules_1();
        seating.apply_rules_1();
        seating.apply_rules_1();
        assert_eq!(seating.count_occupied(), 37);
    }

    #[test]
    fn test_rules_2() {
        let input = "L.LL.LL.LL\nLLLLLLL.LL\nL.L.L..L..\nLLLL.LL.LL\nL.LL.LL.LL\nL.LLLLL.LL\n..L.L.....\nLLLLLLLLLL\nL.LLLLLL.L\nL.LLLLL.LL";
        let mut seating = input.parse::<Seating>().unwrap();

        seating.apply_rules_2();
        assert_eq!(seating.count_occupied(), 71);

        seating.apply_rules_2();
        seating.apply_rules_2();
        seating.apply_rules_2();
        seating.apply_rules_2();
        seating.apply_rules_2();
        assert_eq!(seating.count_occupied(), 26);
    }


}