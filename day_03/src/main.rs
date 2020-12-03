use std::fs;
use std::io::{BufWriter, stdout};

use ferris_says::say;

fn main() {
    let map = read_input();

    let trees_hit = count_trees(map, 3);

    announce_answer(trees_hit.to_string())
}

fn count_trees(map: Vec<String>, right: usize) -> i32 {
    let distance = map.len();
    let map_width = map[0].chars().count();
    let mut x_coord :usize = 0;

    let mut trees_hit = 0;

    for row in 1..distance {
        x_coord = (x_coord + right) % map_width;

        let char = map[row].chars().nth(x_coord).unwrap();
        if char == '#' {
            trees_hit += 1;
        }
    }

    trees_hit
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
    fn test_count_trees() {
        let map = vec!["..##.......".to_string(), "#...#...#..".to_string(), ".#....#..#.".to_string(), "..#.#...#.#".to_string(), ".#...##..#.".to_string(), "..#.##.....".to_string(), ".#.#.#....#".to_string(), ".#........#".to_string(), "#.##...#...".to_string(), "#...##....#".to_string(), ".#..#...#.#".to_string()];
        let count = count_trees(map, 3);
        assert_eq!(7, count);
    }

    #[test]
    fn test_modulo() {
        assert_eq!(0, 2 % 1);
        assert_eq!(1, 3 % 2);
        assert_eq!(0, 0 % 3);

        assert_eq!(3, 7 % 4);
    }
}
