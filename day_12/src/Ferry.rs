use std::str::FromStr;

pub struct Ferry {
    direction_facing: String,
    current_position: (i32, i32)
}

impl Ferry {
    pub fn travel(&mut self, instr: Instruction) {
        if instr.direction_moved.is_some() {
            self.current_position = get_new_position(&self.direction_facing, &self.current_position, instr.direction_moved.unwrap(), instr.distance_moved.unwrap());
        }
        if instr.direction_rotated.is_some() {
            self.direction_facing = get_new_direction_faced(&self.direction_facing, instr.direction_rotated.unwrap(), instr.angle_rotated.unwrap());
        }
    }

    pub fn calc_manhattan_distance(&self) -> i32 {
        self.current_position.0.abs() + self.current_position.1.abs()
    }
}

fn get_new_position(direction_facing: &String, current_position: &(i32, i32), direction_moved: String, distance: i32) -> (i32, i32) {
    let direction_to_move = if direction_moved == "F" { direction_facing } else { &direction_moved };

    let mut new_position =  current_position.clone();

    if direction_to_move == "N" {
        new_position = (new_position.0, new_position.1 + distance);
    } else if direction_to_move == "E" {
        new_position = (new_position.0 + distance, new_position.1);
    } else if direction_to_move == "S" {
        new_position = (new_position.0, new_position.1 - distance);
    } else {
        new_position = (new_position.0 - distance, new_position.1);
    }
    new_position
}

fn get_new_direction_faced(current_direction: &String, direction: String, angle: i32) -> String {
    let directions = ["N", "E", "S", "W"];
    let current_direction_index = directions.iter().position(|&s| s == current_direction).unwrap();
    let directions_moved_through = (angle / 90) as usize;

    let mut new_position_index = current_direction_index;
    if direction == "L" {
        new_position_index += directions.len() - directions_moved_through;
    } else if direction == "R" {
        new_position_index += directions_moved_through;
    }

    new_position_index = new_position_index % directions.len();

    directions[new_position_index].to_string()
}

pub fn new_ferry(direction_facing: &str) -> Ferry {
    Ferry { direction_facing: String::from(direction_facing), current_position: (0, 0) }
}

pub struct Instruction {
    direction_moved: Option<String>,
    distance_moved: Option<i32>,
    direction_rotated: Option<String>,
    angle_rotated: Option<i32>
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let instr = s.chars().nth(0).unwrap().to_string();
        let scalar = s[1..].parse::<i32>().expect("not an integer");

        if instr == "N" || instr == "E" || instr == "S" || instr == "W" || instr == "F" {
            Ok(Instruction{
                direction_moved: Some(instr),
                distance_moved: Some(scalar),
                direction_rotated: None,
                angle_rotated: None
            })
        } else {
            Ok(Instruction {
                direction_moved: None,
                distance_moved: None,
                direction_rotated: Some(instr),
                angle_rotated: Some(scalar)
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut ferry = new_ferry("E");

        ferry.travel("R90".parse::<Instruction>().unwrap());
        assert_eq!("S", ferry.direction_facing);

        ferry.travel("R90".parse::<Instruction>().unwrap());
        assert_eq!("W", ferry.direction_facing);

        ferry.travel("R270".parse::<Instruction>().unwrap());
        assert_eq!("S", ferry.direction_facing);

        ferry.travel("R360".parse::<Instruction>().unwrap());
        assert_eq!("S", ferry.direction_facing);

        ferry.travel("L90".parse::<Instruction>().unwrap());
        assert_eq!("E", ferry.direction_facing);

        ferry.travel("L180".parse::<Instruction>().unwrap());
        assert_eq!("W", ferry.direction_facing);
    }

    #[test]
    fn test_move_ferry() {
        let input = "F10\nN3\nF7\nR90\nF11";
        let instructions: Vec<Instruction> = input.lines().map(|s| s.parse().unwrap()).collect();

        let mut ferry = new_ferry("E");

        for i in instructions {
            ferry.travel(i);
        }

        assert_eq!(25, ferry.calc_manhattan_distance());
    }
}
