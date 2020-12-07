use std::collections::{HashSet};
use std::fs;
use std::io::{BufWriter, stdout};

use ferris_says::say;

fn main() {
    let input = read_input();

    let total_answered_unique_per_group: usize = input.iter()
        .map(|s| count_unique_answered(s))
        .sum();

    let total_answered_everyone_in_group: usize = input.iter()
        .map(|s| count_all_answered_in_group(s))
        .sum();

    announce_answer(format!("{:?} unique questions answered Y per group", total_answered_unique_per_group));
    announce_answer(format!("{:?} questions answered Y by entire groups", total_answered_everyone_in_group));
}

fn count_unique_answered(answered: &str) -> usize {
    let no_whitespace: HashSet<char> = answered.chars().filter(|c| !c.is_whitespace()).collect();
    no_whitespace.len()
}

fn count_all_answered_in_group(answered: &str) -> usize {
    let passenger_answer_sets: Vec<HashSet<char>> = answered.lines()
        .map(|l| l.chars())
        .map(|cs| cs.collect())
        .collect();

    let common_answers: HashSet<&char> = if let Some((first, rest)) = passenger_answer_sets.split_first() {
        rest.iter().fold(hash_to_ref(first), |acc, i| {
            acc.intersection(&hash_to_ref(i)).copied().collect()
        })
    } else {
        HashSet::new()
    };

    return common_answers.len();
}

fn hash_to_ref(h: &HashSet<char>) -> HashSet<&char> {
    h.iter().collect()
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

    #[test]
    fn test_count_all_answered_in_group() {
        assert_eq!(count_all_answered_in_group(&mut "abc".to_string()), 3);
        assert_eq!(count_all_answered_in_group(&mut "a\nb\nc".to_string()), 0);
        assert_eq!(count_all_answered_in_group(&mut "ab\nac".to_string()), 1);
        assert_eq!(count_all_answered_in_group(&mut "a\na\na\na\na".to_string()), 1);
        assert_eq!(count_all_answered_in_group(&mut "b".to_string()), 1);
    }
}