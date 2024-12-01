use std::{env, fs};
use std::collections::HashSet;
use std::path::PathBuf;

fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("src/input");

    let data = load_data(path);

    let solution: Vec<_> = data.iter().map(|(s1, s2)| {
        s1.chars().filter(|&c| s2.contains(c)).collect::<char>()
    }).collect();

    solution.iter().map(|char| {
        if ('A'..='Z').contains(char) {
            char - 74
        } else if ('a'..'z').contains(char) {
            char - 140
        }
    }).sum();



}




fn load_data(path : PathBuf) -> Vec<(String, String)> {
    fs::read_to_string(path).unwrap().lines().map(|line| {
        let (left, right) = line.split_at(line.len() / 2);
        (left.into(), right.into())
    }).collect()
}