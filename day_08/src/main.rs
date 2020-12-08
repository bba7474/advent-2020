use std::fs;
use std::io::{BufWriter, stdout};

use ferris_says::say;

fn main() {
    let input = read_input();
    let acc_at_loop_repeat = find_acc_at_infinite_loop(input.clone());
    announce_answer(format!("When re-entering the infinite loop, the acc value is {}", acc_at_loop_repeat));
    let acc_for_winning_instruction = get_acc_value_for_winning_route(input.clone());
    announce_answer(format!("With a working instruction set, the acc value is {}", acc_for_winning_instruction));
}

fn find_acc_at_infinite_loop(instructions: Vec<String>) -> i32 {
    let mut acc = 0;
    let mut index = 0;
    let mut indexes_visited = vec![];

    loop {
        if indexes_visited.contains(&index) {
            break;
        }
        indexes_visited.push(index);

        let command = instructions.get(index).unwrap();
        let split = command.split(" ").collect::<Vec<&str>>();

        let action = &split[0];
        let size = split[1][1..].parse::<usize>().expect("is not an integer");
        let direction = split[1].chars().nth(0).unwrap();
        if action == &"jmp" {
            if direction == '+' {
                index += size;
            } else {
                index -= size;
            }
            continue;
        }
        if action == &"acc" {
            if direction == '+' {
                acc += size as i32;
            } else {
                acc -= size as i32;
            }
        }
        index += 1;
    }

    acc
}

fn get_acc_value_for_winning_route(base_instructions: Vec<String>) -> i32 {
    let mut acc = 0;
    let mut index = 0;
    let mut indexes_visited = vec![];

    let instruction_length = base_instructions.len();

    let mut current_instructions = base_instructions;
    let mut winner_found = false;

    loop {
        if indexes_visited.contains(&index) {
            break;
        }
        if index >= instruction_length {
            break;
        }
        indexes_visited.push(index);

        if !winner_found {
            let command_to_change = current_instructions[index].clone();
            let to_change = command_to_change.split(" ").collect::<Vec<&str>>();
            if to_change[0] == "jmp" {
                let mut new_instructions = current_instructions.clone();
                new_instructions[index] = format!("{} {}", "nop", to_change[1]);
                let wins = does_win(index, &new_instructions);
                if wins {
                    current_instructions = new_instructions;
                    winner_found = true;
                }
            } else if to_change[0] == "nop" {
                let mut new_instructions = current_instructions.clone();
                new_instructions[index] = format!("{} {}", "jmp", to_change[1]);
                let wins = does_win(index, &new_instructions);
                if wins {
                    current_instructions = new_instructions;
                    winner_found = true;
                }
            }
        }

        let command = current_instructions[index].clone();
        let split = command.split(" ").collect::<Vec<&str>>();

        let action = &split[0];

        let size = split[1][1..].parse::<usize>().expect("is not an integer");
        let direction = split[1].chars().nth(0).unwrap();
        if action == &"jmp" {
            if direction == '+' {
                index += size;
            } else {
                index -= size;
            }
            continue;
        }
        if action == &"acc" {
            if direction == '+' {
                acc += size as i32;
            } else {
                acc -= size as i32;
            }
        }
        index += 1;
    }

    acc
}

fn does_win(start_index: usize, modified_instructions: &Vec<String>) -> bool {
    let mut index = start_index;

    let mut indexes_visited = vec![];
    let target_index = modified_instructions.len();

    loop {
        if indexes_visited.contains(&index) {
            return false;
        }
        if index == target_index {
            return true;
        }
        indexes_visited.push(index);

        let command = modified_instructions.get(index).unwrap();
        let split = command.split(" ").collect::<Vec<&str>>();

        let action = &split[0];
        let size = split[1][1..].parse::<usize>().expect("is not an integer");
        let direction = split[1].chars().nth(0).unwrap();
        if action == &"jmp" {
            if direction == '+' {
                index += size;
            } else {
                index -= size;
            }
            continue;
        }
        index += 1;
    }
}

fn read_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .expect("Error reading file")
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn announce_answer(answer: String) {
    let message = format!("{}", answer).to_string();
    let stdout = stdout();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

