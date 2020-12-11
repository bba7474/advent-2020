use std::fs;
use std::io::{BufWriter, stdout};

use ferris_says::say;

fn main() {
    let mut input = read_input();
    input.sort();
    input.push(input.last().unwrap() + 3); // the final jolt to device

    let (jolts_1, jolts_3) = count_jolts(&input);
    announce_answer(format!("{} jolts of 1, {} jolts of 3, answer is {}", jolts_1, jolts_3, jolts_1 * jolts_3));

    let count_adapter_configs = count_adapter_configs(&input);
    announce_answer(format!("There are {} different adapter configurations", count_adapter_configs));
}

fn count_jolts(sorted_adapters: &Vec<i32>) -> (i32, i32) {
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
        current_adapter = adapter.clone();
    }
    return (jolts_1, jolts_3);
}

fn count_adapter_configs(sorted_adapters: &Vec<i32>) -> i64 {
    let length = sorted_adapters.len();

    let mut configs_to_adapter_at_index= Vec::new();

    for i in 0..length {
        let adapter_at_i = sorted_adapters[i];
        let mut configs_to_i = if adapter_at_i <= 3 { 1 } else { 0 };

        let can_step_from_with_index_back = sorted_adapters[..i].iter().rev().filter(|&e| e + 3 >= adapter_at_i).enumerate();
        for (j, _adapter) in can_step_from_with_index_back {
            configs_to_i += configs_to_adapter_at_index[i - (j + 1)]
        }
        configs_to_adapter_at_index.push(configs_to_i);
    }

    configs_to_adapter_at_index[sorted_adapters.len() - 1]
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


#[cfg(test)]
mod tests {

    #[test]
    fn test_get_vector_indexes() {
        let vector = vec![0, 2, 3, 4, 5];
        let i = vector.len() - 1;

        for (j, value) in vector[..i].iter().rev().filter(|&e| e + 3 >= vector[i]).enumerate() {
            if j == 0 {
                assert_eq!(value, &4);
            } else if j == 1 {
                assert_eq!(value, &3);
            } else if j == 2 {
                assert_eq!(value, &2);
            } else {
                panic!("Should not be here")
            }
        }
    }
}

