use std::fs;
use std::io::{BufWriter, stdout};

use ferris_says::say;
use crate::luggage::{Luggage, LuggageContainsRules};
use std::collections::HashMap;

mod luggage;

fn main() {
    let input = read_input();

    let luggage_rules: HashMap<String, Vec<LuggageContainsRules>> = input.iter()
        .map(|s| s.parse::<Luggage>().unwrap())
        .map(|l| l.get_entry())
        .collect();

    let wanted_color = "shiny gold".to_string();

    let bags_containing_color = luggage_rules.iter()
        .filter(|(bag, _rule)| bag_can_contain_bag(bag.to_string(), &wanted_color, &luggage_rules))
        .count();

    let bags_contained_by = count_bags_contained_in(&wanted_color, &luggage_rules);

    announce_answer(format!("{:?} different color bags can contain a {} bag", bags_containing_color, wanted_color));
    announce_answer(format!("{:?} bags a contained within a {} bag", bags_contained_by, wanted_color));
}

fn bag_can_contain_bag(bag: String, can_contain: &String, luggage_rules: &HashMap<String, Vec<LuggageContainsRules>>) -> bool {
    let rule = luggage_rules.get(&bag).unwrap();
    if rule.iter().any(|r| &r.get_color() == can_contain) {
        return true;
    }
    if rule.len() == 0 {
        return false;
    }
    return rule.iter().any(|new_bag| bag_can_contain_bag(new_bag.get_color(), can_contain, luggage_rules));
}

fn count_bags_contained_in(bag: &String, luggage_rules: &HashMap<String, Vec<LuggageContainsRules>>) -> i32 {
    let rules = luggage_rules.get(bag).unwrap();

    let mut num_bags = 0;
    for rule in rules {
        num_bags += rule.get_count(); // the bags themselves
        num_bags += rule.get_count() * count_bags_contained_in(&rule.get_color(), luggage_rules); // the bags contained
    }

    num_bags
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
