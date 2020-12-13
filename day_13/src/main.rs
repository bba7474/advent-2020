use std::fs;
use std::io::{BufWriter, stdout};
use itertools::{iproduct};

use ferris_says::say;

fn main() {
    let input_1 = read_input_1();

    let part_1_output = get_bus_id_and_wait_time_once_arrived(input_1.0, input_1.1);
    announce_answer(format!("Get bus {} after waiting {} minutes: {}", part_1_output.0, part_1_output.1, part_1_output.0 * part_1_output.1));

    let input_2 = read_input_2();
    assert_are_pairwise_coprime(input_2.iter().map(|(_, id)| id.clone()).collect()); // can use Chinese Remainder Theorem

    let time_with_buses_departing_at_offsets = calc_time_with_buses_departing_at_offsets_by_crt(
        input_2.iter().map(|(_, id)| id.clone()).collect(),
        input_2.iter().map(|(offset, id)| id - offset).collect()
    );

    announce_answer(format!("{} is the first time where buses depart at the desired offsets", time_with_buses_departing_at_offsets));
}

fn calc_time_with_buses_departing_at_offsets_by_crt(numbers: Vec<i64>, remainders: Vec<i64>) -> i64 {
    let number_product = numbers.iter().product::<i64>();
    let mut partial_products = vec![0; numbers.len()];
    let mut inverses = vec![0; numbers.len()];

    let mut sum = 0_i64;

    for (i, n) in numbers.iter().enumerate() {
        partial_products[i] = number_product / n;
        inverses[i] = compute_inverse(&partial_products[i], &numbers[i]);
        sum += partial_products[i] * inverses[i] * remainders[i];
    }

    return sum % number_product;
}

fn compute_inverse(i1: &i64, i2: &i64) -> i64 {
    let mut a = i1.clone();
    let mut b = i2.clone();
    let m = b;
    let mut t: i64;
    let mut q: i64;
    let mut x = 0_i64;
    let mut y = 1_i64;

    if b == 1 {
        return 0_i64;
    }

    while a > 1_i64 {
        q = a / b;
        t = b;

        b = a % b;
        a = t;
        t = x;

        x = y - q * x;
        y = t;
    }
    if y < 0 {
        y += m;
    }
    return y;
}

fn assert_are_pairwise_coprime(ints: Vec<i64>) {
    assert!(
        iproduct!(ints.iter().clone(), ints.iter().clone())
            .all(|(i1, i2)| i1 == i2 || gcd_by_euclid(i1.clone(), i2.clone()) == 1)
    )
}

fn gcd_by_euclid(i1: i64, i2: i64) -> i64 {
    if i2 == 0 {
        return i1;
    }
    gcd_by_euclid(i2, i1 % i2)
}

fn get_bus_id_and_wait_time_once_arrived(arrival_time: i32, bus_ids: Vec<i32>) -> (i32, i32) {
    bus_ids.iter()
        .map(|id| (id.clone(), id - (arrival_time % id)))
        .min_by(|a, b| a.1.cmp(&b.1)).unwrap()
}

fn read_input_1() -> (i32, Vec<i32>) {
    let input_lines = fs::read_to_string("input.txt")
        .expect("Error reading file")
        .lines()
        .map(|s| String::from(s))
        .collect::<Vec<String>>();

    let earliest_time: i32 = input_lines[0].parse().expect("not an integer");
    let bus_ids: Vec<i32> = input_lines[1].split(",").filter(|s| s != &"x").map(|s| s.parse().expect("not an integer")).collect();
    (earliest_time, bus_ids)
}

fn read_input_2() -> Vec<(i64, i64)> {
    let input_lines = fs::read_to_string("input.txt")
        .expect("Error reading file")
        .lines()
        .map(|s| String::from(s))
        .collect::<Vec<String>>();

    input_lines[1].split(",").enumerate().filter(|(_, j)| j != &"x").map(|(i, j)| (i as i64, j.parse().expect("not an int"))).collect()
}

fn announce_answer(answer: String) {
    let message = format!("{}", answer).to_string();
    let stdout = stdout();
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_bus_to_catch() {
        let output = get_bus_id_and_wait_time_once_arrived(939, vec![7, 13, 59, 31, 19]);
        assert_eq!(59, output.0);
        assert_eq!(5, output.1);
    }

    #[test]
    fn test_time_with_buses_at_offsets() {
        let time = calc_time_with_buses_departing_at_offsets_by_crt(
            vec![7, 13, 59, 31, 19],
            vec![0, 12, 55, 25, 12]
        );
        assert_eq!(1068781, time);
    }

    #[test]
    fn test_time_with_buses_at_offsets_crt() {
        let time = calc_time_with_buses_departing_at_offsets_by_crt(
            vec![67, 7, 59, 61],
            vec![0, 6, 57, 58],
        );
        assert_eq!(754018, time);
    }
}
