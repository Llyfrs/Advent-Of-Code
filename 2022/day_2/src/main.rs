use std::{env, fs};
use std::path::PathBuf;

fn main() {

    let mut path = env::current_dir().unwrap();
    path.push("src/input");

    let data = load_data(path);

    // A X - Rock
    // B Y - Paper
    // C Z - Scissors

    let result1 :i32 = data.iter().map(|turn| {
        (match turn {
            ('A' , 'X') | ('B','Y') | ('C', 'Z') => 3,
            ('A' , 'Y') | ('B','Z') | ('C', 'X') => 6,
            ('A' , 'Z') | ('B','X') | ('C', 'Y') => 0,
            _ => 0
        }) + (match turn {
            (_,'X') => 1,
            (_,'Y') => 2,
            (_,'Z') => 3,
            _ => 0
        })
    }).sum();

    // A - Rock     X - Lose
    // B - Paper    Y - Draw
    // C - Scissors Z - Win

    let chose = | rock, win : bool | {
        match rock {
            'A' => if win {2} else {3},
            'B' => if win {3} else {1},
            'C' => if win {1} else {2},
            _ => 0
        }
    };

    let result2 : i32 = data.iter().map(|turn| {
        match turn {
            (x, 'Z') => chose(*x, true) + 6,
            (x, 'X') => chose(*x, false),
            (x, 'Y') => (if *x == 'A' {1} else if *x == 'B' {2} else {3}) + 3,
            _=> 0
        }
    }).sum();

    println!("Day 2 part 1: {result1}"); // 10310
    println!("Day 2 part 1: {result2}"); // 14859
}



fn load_data(path :PathBuf) -> Vec<(char, char)> {
    fs::read_to_string(path).unwrap().lines().map(|line| {
        (line.chars().nth(0).unwrap(), line.chars().nth(2).unwrap())
    }).collect()
}