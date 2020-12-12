use std::str::FromStr;

pub struct Ferry {
    direction_facing: String,
    current_position: (i32, i32),
    waypoint_relative_position: (i32, i32),
}

impl Ferry {
    pub fn travel_1(&mut self, instr: Instruction1) {
        if instr.direction_moved.is_some() {
            self.current_position = get_new_position(&self.direction_facing, &self.current_position, instr.direction_moved.unwrap(), instr.distance_moved.unwrap());
        }
        if instr.direction_rotated.is_some() {
            self.direction_facing = get_new_direction_faced(&self.direction_facing, instr.direction_rotated.unwrap(), instr.angle_rotated.unwrap());
        }
    }

    pub fn travel_2(&mut self, instr: Instruction2) {
        if instr.to_waypoint_times.is_some() {
            self.current_position = travel_to_waypoint(&self.current_position, &self.waypoint_relative_position, instr.to_waypoint_times.unwrap());
        }
        if instr.waypoint_move_dir.is_some() {
            self.waypoint_relative_position = calc_new_position(&self.waypoint_relative_position, &instr.waypoint_move_dir.unwrap(), instr.waypoint_move_dist.unwrap());
        }
        if instr.waypoint_rotate_dir.is_some() {
            self.waypoint_relative_position = rotate_position(&self.waypoint_relative_position, &instr.waypoint_rotate_dir.unwrap(), instr.waypoint_rotate_angle.unwrap());
        }
    }

    pub fn calc_manhattan_distance(&self) -> i32 {
        self.current_position.0.abs() + self.current_position.1.abs()
    }
}

fn rotate_position(current_position: &(i32, i32), rotate_dir: &String, rotate_angle: i32) -> (i32, i32) {
    let mut new_position = current_position.clone();

    if rotate_angle == 180 {
        new_position = (-new_position.0, -new_position.1);
    } else if (rotate_dir == "R" && rotate_angle == 90) || (rotate_dir == "L" && rotate_angle == 270) {
        new_position = (new_position.1, -new_position.0);
    } else if (rotate_dir == "R" && rotate_angle == 270) || (rotate_dir == "L" && rotate_angle == 90) {
        new_position = (-new_position.1, new_position.0);
    }

    new_position
}

fn get_new_position(direction_facing: &String, current_position: &(i32, i32), direction_moved: String, distance: i32) -> (i32, i32) {
    let direction_to_move = if direction_moved == "F" { direction_facing } else { &direction_moved };
    calc_new_position(current_position, direction_to_move, distance)
}

fn calc_new_position(current_position: &(i32, i32), direction_to_move: &String, distance: i32) -> (i32, i32) {
    let mut new_position = current_position.clone();

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

fn travel_to_waypoint(current_position: &(i32, i32), waypoint_relative_pos: &(i32, i32), times_to_waypoint: i32) -> (i32, i32) {
    (current_position.0 + (waypoint_relative_pos.0 * times_to_waypoint), current_position.1 + (waypoint_relative_pos.1 * times_to_waypoint))
}

pub fn new_ferry(direction_facing: &str) -> Ferry {
    Ferry { direction_facing: String::from(direction_facing), current_position: (0, 0), waypoint_relative_position: (10, 1) }
}

pub struct Instruction1 {
    direction_moved: Option<String>,
    distance_moved: Option<i32>,
    direction_rotated: Option<String>,
    angle_rotated: Option<i32>,
}

impl FromStr for Instruction1 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instr = s.chars().nth(0).unwrap().to_string();
        let scalar = s[1..].parse::<i32>().expect("not an integer");

        if instr == "N" || instr == "E" || instr == "S" || instr == "W" || instr == "F" {
            Ok(Instruction1 {
                direction_moved: Some(instr),
                distance_moved: Some(scalar),
                direction_rotated: None,
                angle_rotated: None,
            })
        } else {
            Ok(Instruction1 {
                direction_moved: None,
                distance_moved: None,
                direction_rotated: Some(instr),
                angle_rotated: Some(scalar),
            })
        }
    }
}

pub struct Instruction2 {
    to_waypoint_times: Option<i32>,
    waypoint_move_dir: Option<String>,
    waypoint_move_dist: Option<i32>,
    waypoint_rotate_dir: Option<String>,
    waypoint_rotate_angle: Option<i32>,
}

impl FromStr for Instruction2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instr = s.chars().nth(0).unwrap().to_string();
        let scalar = s[1..].parse::<i32>().expect("not an integer");

        if instr == "F" {
            Ok(Instruction2 {
                to_waypoint_times: Some(scalar),
                waypoint_move_dir: None,
                waypoint_move_dist: None,
                waypoint_rotate_dir: None,
                waypoint_rotate_angle: None,
            })
        } else if instr == "N" || instr == "E" || instr == "S" || instr == "W" {
            Ok(Instruction2 {
                to_waypoint_times: None,
                waypoint_move_dir: Some(instr),
                waypoint_move_dist: Some(scalar),
                waypoint_rotate_dir: None,
                waypoint_rotate_angle: None,
            })
        } else {
            Ok(Instruction2 {
                to_waypoint_times: None,
                waypoint_move_dir: None,
                waypoint_move_dist: None,
                waypoint_rotate_dir: Some(instr),
                waypoint_rotate_angle: Some(scalar),
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

        ferry.travel_1("R90".parse::<Instruction1>().unwrap());
        assert_eq!("S", ferry.direction_facing);

        ferry.travel_1("R90".parse::<Instruction1>().unwrap());
        assert_eq!("W", ferry.direction_facing);

        ferry.travel_1("R270".parse::<Instruction1>().unwrap());
        assert_eq!("S", ferry.direction_facing);

        ferry.travel_1("R360".parse::<Instruction1>().unwrap());
        assert_eq!("S", ferry.direction_facing);

        ferry.travel_1("L90".parse::<Instruction1>().unwrap());
        assert_eq!("E", ferry.direction_facing);

        ferry.travel_1("L180".parse::<Instruction1>().unwrap());
        assert_eq!("W", ferry.direction_facing);
    }

    #[test]
    fn test_move_ferry_1() {
        let input = "F10\nN3\nF7\nR90\nF11";
        let instructions: Vec<Instruction1> = input.lines().map(|s| s.parse().unwrap()).collect();

        let mut ferry = new_ferry("E");

        for i in instructions {
            ferry.travel_1(i);
        }

        assert_eq!(25, ferry.calc_manhattan_distance());
    }

    #[test]
    fn test_move_ferry_2() {
        let input = "F10\nN3\nF7\nR90\nF11";
        let instructions: Vec<Instruction2> = input.lines().map(|s| s.parse().unwrap()).collect();

        let mut ferry = new_ferry("E");

        for i in instructions {
            ferry.travel_2(i);
        }

        assert_eq!(286, ferry.calc_manhattan_distance());
    }
}
