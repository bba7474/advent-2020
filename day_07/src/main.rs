use std::fs;
use std::io::{BufWriter, stdout};

use ferris_says::say;
use crate::luggage::Luggage;
use std::collections::HashMap;

mod luggage;

fn main() {
    let input = read_input();

    let luggage_rules: HashMap<String, Vec<String>> = input.iter()
        .map(|s| s.parse::<Luggage>().unwrap())
        .map(|l| l.get_entry())
        .collect();

    let bags_containing_color = luggage_rules.iter()
        .filter(|(bag, _rule)| bag_can_contain_bag(bag.to_string(), "shiny gold".to_string(), &luggage_rules))
        .count();

    announce_answer(bags_containing_color.to_string());
}

fn bag_can_contain_bag(bag: String, can_contain: String, luggage_rules: &HashMap<String, Vec<String>>) -> bool {
    let rule = luggage_rules.get(&bag).unwrap();
    if rule.contains(&can_contain) {
        return true;
    }
    if rule.len() == 0 {
        return false;
    }
    return rule.iter().any(|new_bag| bag_can_contain_bag(new_bag.to_string(), can_contain.clone(), luggage_rules));
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
