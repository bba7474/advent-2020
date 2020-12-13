use std::fs;
use std::io::{BufWriter, stdout};

use ferris_says::say;

fn main() {
    let input = read_input();

    let part_1_output = get_bus_id_and_wait_time_once_arrived(input.0, input.1);

    announce_answer(format!("Get bus {} after waiting {} minutes: {}", part_1_output.0, part_1_output.1, part_1_output.0 * part_1_output.1));
}

fn get_bus_id_and_wait_time_once_arrived(arrival_time: i32, bus_ids: Vec<i32>) -> (i32, i32) {
    bus_ids.iter()
        .map(|id| (id.clone(), id - (arrival_time % id)))
        .min_by(|a, b| a.1.cmp(&b.1)).unwrap()
}

fn read_input() -> (i32, Vec<i32>) {
    let input_lines = fs::read_to_string("input.txt")
        .expect("Error reading file")
        .lines()
        .map(|s| String::from(s))
        .collect::<Vec<String>>();

    let earliest_time: i32 = input_lines[0].parse().expect("not an integer");
    let bus_ids: Vec<i32> = input_lines[1].split(",").filter(|s| s != &"x").map(|s| s.parse().expect("not an integer")).collect();
    (earliest_time, bus_ids)
}

fn announce_answer(answer: String) {
    let message = format!("{}", answer).to_string();
    let stdout = stdout();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_bus_to_catch() {
        let output = get_bus_id_and_wait_time_once_arrived(939, vec![7, 13, 59, 31, 19]);
        assert_eq!(59, output.0);
        assert_eq!(5, output.1);
    }
}
