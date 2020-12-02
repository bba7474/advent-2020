use std::fs;
use std::io::{BufWriter, stdout};

use ferris_says::say;

fn main() {
    let password_list = read_input();
    let valid_password_count = count_valid_pwds(password_list);
    announce_answer(valid_password_count.to_string());
}

fn count_valid_pwds(pwd_rules: Vec<String>) -> i32 {
    let mut valid_count = 0;

    for pwd_rule in pwd_rules {
        let splitted = split(pwd_rule, " ".to_string());

        let wanted_indexes = split(splitted[0].to_string(), "-".to_string());
        let index_1 = wanted_indexes[0].parse::<i32>().expect("not integer") - 1;
        let index_2 = wanted_indexes[1].parse::<i32>().expect("not integer") - 1;

        let expected_letter = &splitted[1].chars().nth(0).unwrap();
        let password = &splitted[2];

        if (password.chars().nth(index_1 as usize).unwrap() == *expected_letter) ^ (password.chars().nth(index_2 as usize).unwrap() == *expected_letter) {
            valid_count += 1;
        }
    }

    return valid_count;
}

fn split(to_split: String, split_on: String) -> Vec<String> {
    to_split.split(&split_on).map(|s| s.to_string()).collect()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_valid_pwds() {
        let count = count_valid_pwds(vec!["1-3 a: abcde".to_string(), "1-3 b: cdefg".to_string(), "2-9 c: ccccccccc".to_string()]);
        assert_eq!(1, count);
    }
}