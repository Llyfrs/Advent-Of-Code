use std::iter::zip;
use std::path::absolute;

fn load_input() -> (Vec<i32>, Vec<i32>) {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split("   ");
        let left = parts.next().unwrap();
        let right = parts.next().unwrap();
        left_list.push(left.parse().unwrap());
        right_list.push(right.parse().unwrap());
    }

    (left_list, right_list)
}

fn main() {
    println!("Hello, world!");

    // PART 1
    let (mut left_list, mut right_list) = load_input();
    left_list.sort();
    right_list.sort();

    let difrence : i32 = zip(left_list.iter(), right_list.iter())
        .map(|(left, right)| {
           (left-right).abs()
        }).sum();

    println!("Difrence: {}", difrence);

    // PART 2
    let similarity : i32 = zip(left_list.iter(), right_list.iter())
        .map(|(left, right)| {
            right_list.iter().filter(| &x| left == x).count() as i32 * left
        }).sum();

    println!("similarity: {}", similarity);
}
