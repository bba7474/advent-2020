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

    for pwd in pwd_rules {
        let splitted = split(pwd, " ".to_string());

        let min_max = split(splitted[0].to_string(), "-".to_string());
        let min = min_max[0].parse::<i32>().expect("not integer");
        let max = min_max[1].parse::<i32>().expect("not integer");

        let expected_letter = &splitted[1][..1];
        let password = &splitted[2];

        let matches = password.matches(expected_letter).count();

        if min <= matches as i32 && matches as i32 <= max {
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
        assert_eq!(2, count);
    }
}