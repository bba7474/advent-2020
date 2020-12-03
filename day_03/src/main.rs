use std::fs;
use std::io::{BufWriter, stdout};

use ferris_says::say;

fn main() {
    let map = read_input();

    let right_1_down_1 = count_trees(&map, 1, 1);
    let right_5_down_1 = count_trees(&map, 5, 1);
    let right_7_down_1 = count_trees(&map, 7, 1);
    let right_1_down_2 = count_trees(&map, 1, 2);
    let right_3_down_1 = count_trees(&map, 3, 1);

    let product = right_1_down_1 * right_3_down_1 * right_5_down_1 * right_7_down_1 * right_1_down_2;

    announce_answer(product.to_string())
}

fn count_trees(map: &Vec<String>, right: usize, down: usize) -> i64 {
    let distance = map.len();
    let map_width = map[0].chars().count();
    let mut x_coord = 0;

    let mut trees_hit = 0;

    for row in (0..distance).step_by(down) {
        let char = map[row].chars().nth(x_coord).unwrap();
        if char == '#' {
            trees_hit += 1;
        }

        x_coord = (x_coord + right) % map_width;
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
        assert_eq!(2, count_trees(&map, 1, 1));
        assert_eq!(7, count_trees(&map, 3, 1));
        assert_eq!(3, count_trees(&map, 5, 1));
        assert_eq!(4, count_trees(&map, 7, 1));
        assert_eq!(2, count_trees(&map, 1, 2));
    }
}
