use std::fs;
use std::io::{BufWriter, stdout};

use ferris_says::say;
use itertools::{iproduct};

fn main() {
    let expense_list = read_input();

    let product = calc_expenses(expense_list);

    if product != 0 {
        print(format!("The answer is: {}", product));
    } else {
        print("No answer found".to_string());
    }
}

fn read_input() -> Vec<i32> {
    fs::read_to_string("input.txt")
        .expect("Error reading file")
        .lines()
        .map(|e| e.parse::<i32>().expect("not integer"))
        .collect()
}

fn calc_expenses(expense_list: Vec<i32>) -> i32 {
    iproduct!(
        expense_list.iter().cloned(),
        expense_list.iter().cloned(),
        expense_list.iter().cloned()
    )
        .find(|(x, y, z)| x + y + z == 2020)
        .map(|(x, y, z)| x * y * z)
        .unwrap_or(0)
}

fn print(message: String) {
    let stdout = stdout();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_expenses() {
        let product = calc_expenses(vec![1721, 979, 366, 299, 675, 1456]);
        assert_eq!(241861950, product);
    }

    #[test]
    fn test_calc_expenses_no_match() {
        let product = calc_expenses(vec![5, 2020, 1000]);
        assert_eq!(0, product);
    }

    #[test]
    fn test_itertools() {
        let x = vec![1, 2, 3];
        let product = iproduct!(x.iter().cloned(), x.iter().cloned());
        product.for_each(|x| println!("{:?}", x));
    }

}
