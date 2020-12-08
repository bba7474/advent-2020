use std::fs;
use std::io::{BufWriter, stdout};

use ferris_says::say;

fn main() {
    let input = read_input();
    let acc_at_loop_repeat = find_acc_at_infinite_loop(input);
    announce_answer(acc_at_loop_repeat.to_string());
}

fn find_acc_at_infinite_loop(instructions: Vec<String>) -> i32 {
    let mut acc = 0;
    let mut index = 0;
    let mut indexes_visited = vec![];

    loop {
        // do the thing
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

fn read_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .expect("Error reading file")
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn announce_answer(answer: String) {
    let message = format!("The answer is: {}", answer).to_string();
    let stdout = stdout();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

