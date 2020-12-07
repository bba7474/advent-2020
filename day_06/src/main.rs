use std::fs;
use std::io::{BufWriter, stdout};

use ferris_says::say;

fn main() {
    let input = read_input();

     let total_answered_y: usize = input.iter()
        .map(|s| count_unique_answered(s))
        .sum();

    announce_answer(total_answered_y.to_string())
}

fn count_unique_answered(answered: &str) -> usize {
    let mut no_whitespace = answered.chars().filter(|c| !c.is_whitespace()).collect::<Vec<char>>();
    no_whitespace.sort();
    no_whitespace.dedup();
    no_whitespace.len()
}

fn read_input() -> Vec<String> {
    fs::read_to_string("input.txt")
        .expect("Error reading file")
        .split("\r\n\r\n")
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
    fn test_count_unique_answered() {
        assert_eq!(count_unique_answered(&mut "abc".to_string()), 3);
        assert_eq!(count_unique_answered(&mut "a\nb\nc".to_string()), 3);
        assert_eq!(count_unique_answered(&mut "ab\nac".to_string()), 3);
        assert_eq!(count_unique_answered(&mut "a\na\na\na\na".to_string()), 1);
        assert_eq!(count_unique_answered(&mut "b".to_string()), 1);
    }
}