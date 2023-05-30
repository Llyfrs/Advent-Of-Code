#![allow(non_snake_case)]

use std::{env, fs};
use std::path::{PathBuf};

fn main() {

    let mut path = env::current_dir().unwrap();
    path.push("src/input-1");

    let data = get_files(path);

    let solution1: u32 = data
        .iter()
        .map(|elf| {elf.iter().sum()})
        .max()
        .unwrap();

    let mut solution2: Vec<u32> = data.iter()
        .map(|elf| {elf.iter().sum()}).collect();

    solution2.sort();
    solution2.reverse();

    let solution2 : u32 = solution2[0] + solution2[1] + solution2[2];

    println!("Day 1 Part 1 {solution1}");
    println!("Day 1 Part 2 {solution2}");

}

fn get_files(path: PathBuf) -> Vec<Vec<u32>> {

    let content = fs::read_to_string(path)
        .expect("Not able to read the file");

    let var :Vec<Vec<u32>> = content.split("\n\n").map(| chunk | {
        chunk.lines().map(|line| {
            line.parse().unwrap()
        }).collect()
    }).collect();

    return var;
}