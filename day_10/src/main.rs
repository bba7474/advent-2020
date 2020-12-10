use ferris_says::say;
use std::fs;
use std::io::{BufWriter, stdout};

fn main() {
    let mut input = read_input();

    input.sort();

    let (jolts_1, mut jolts_3) = count_jolts(input);

    jolts_3 += 1; // account for the jolt ot the device

    announce_answer(format!("{} jolts of 1, {} jolts of 3, answer is {}", jolts_1, jolts_3, jolts_1 * jolts_3));
}

fn count_jolts(sorted_adapters: Vec<i32>) -> (i32, i32) {
    let mut current_adapter = 0;
    let mut jolts_1 = 0;
    let mut jolts_3 = 0;
    for adapter in sorted_adapters {
        let jolt = adapter - current_adapter;
        if jolt == 1 {
            jolts_1 += 1;
        }
        if jolt == 3 {
            jolts_3 += 1;
        }
        current_adapter = adapter;
    }
    return (jolts_1, jolts_3);
}

fn read_input() -> Vec<i32> {
    fs::read_to_string("input.txt")
        .expect("Error reading file")
        .lines()
        .map(|s| s.parse().expect("not an integer"))
        .collect()
}

fn announce_answer(answer: String) {
    let message = format!("{}", answer).to_string();
    let stdout = stdout();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}
