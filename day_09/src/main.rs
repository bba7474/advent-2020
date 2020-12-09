use std::fs;
use std::io::{BufWriter, stdout};

use ferris_says::say;
use itertools::iproduct;

fn main() {
    let input = read_input();
    let invalid_number = find_number_not_matching_rule(input.clone());
    announce_answer(format!("{} is the first number that is not a sum of two of the previous 25 numbers", invalid_number));
    let contiguous_sum_list = get_contiguous_list_summing_to(invalid_number, input.clone());
    let min = contiguous_sum_list.iter().min().unwrap();
    let max = contiguous_sum_list.iter().max().unwrap();
    announce_answer(format!("{} is the sum of the min and max of the contiguous list that sums to the invalid number {}", min + max, invalid_number));
}

fn find_number_not_matching_rule(xmas_data: Vec<i64>) -> i64 {
    for i in 25..xmas_data.len() - 1 {

        let to_sum = xmas_data[i - 25..i].to_vec();

        if !has_pair_summing_to(xmas_data[i], to_sum) {
            return xmas_data[i];
        }
    }

    return 0;
}

fn has_pair_summing_to(target: i64, to_sum: Vec<i64>) -> bool {
    iproduct!(to_sum.iter().cloned(), to_sum.iter().cloned())
        .any(|(x, y)| x != y && x + y == target)
}

fn get_contiguous_list_summing_to(target: i64, xmas_data: Vec<i64>) -> Vec<i64> {
    for i in 0..xmas_data.len() - 1 {
        let mut sum = 0;
        let mut c_list = Vec::new();
        for j in i..xmas_data.len() - 1 {
            let value = xmas_data[j];
            sum += value;
            c_list.push(value);
            if sum >= target {
                break;
            }
        }
        if sum == target {
            return c_list;
        }
    }

    return Vec::new();
}

fn read_input() -> Vec<i64> {
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
