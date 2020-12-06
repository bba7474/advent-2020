use std::str::FromStr;

pub fn get_plane_seat_id(seat_str: &str) -> i32 {
    let seat: PlaneSeat = seat_str.parse().unwrap();
    return seat.get_id()
}

struct PlaneSeat {
    row: i32,
    seat: i32
}

impl PlaneSeat {
    pub fn get_id(&self) -> i32 {
        return (self.row * 8) + self.seat;
    }
}

impl FromStr for PlaneSeat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut min_row = 0.0_f64;
        let mut max_row = 127.0_f64;

        let mut min_col = 0.0_f64;
        let mut max_col = 7.0_f64;

        let row_str = &s[0..7];
        let col_str = &s[7..];

        for row_char in row_str.chars() {
            let diff: f64 = (max_row - min_row) / 2.0;
            if row_char == 'F' {
                max_row = (max_row - diff).floor();
            }
            if row_char == 'B' {
                min_row = (min_row + diff).ceil();
            }
        }

        assert_eq!(min_row, max_row);

        for col_char in col_str.chars() {
            let diff: f64 = (max_col - min_col) / 2.0;
            if col_char == 'L' {
                max_col = (max_col - diff).floor();
            }
            if col_char == 'R' {
                min_col = (min_col + diff).ceil();
            }
        }

        assert_eq!(min_col, max_col);

        Ok(PlaneSeat {
            row: min_row as i32,
            seat: min_col as i32
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tst_get_seat_id() {
        assert_eq!("FBFBBFFRLR".parse::<PlaneSeat>().unwrap().get_id(), 357);
        assert_eq!("BFFFBBFRRR".parse::<PlaneSeat>().unwrap().get_id(), 567);
        assert_eq!("FFFBBBFRRR".parse::<PlaneSeat>().unwrap().get_id(), 119);
        assert_eq!("BBFFBBFRLL".parse::<PlaneSeat>().unwrap().get_id(), 820);
    }

}