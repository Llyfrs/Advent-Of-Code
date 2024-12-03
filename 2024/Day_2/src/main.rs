use std::arch::x86_64::_bittest;

fn load_input() -> Vec<Vec<i32>> {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_safe(level: Vec<i32>) -> bool {
    let zip = level.iter().zip(level.iter().skip(1));
    let diff = level[0] - level[1];
    for (a, b) in zip {

        let temp = a - b;
        if temp.signum() != diff.signum() || temp.abs() > 3 {
            return false;
        }
    }

    true
}

// Part 2 (Bit brute forced)
fn is_safe_with_dampener(level: Vec<i32>) -> bool {
    for i in 0..level.len() + 1 {

        let mut copy = level.clone();

        if i != 0  {
            copy.remove(i-1);
        }

        if is_safe(copy.clone()) {
            return true;
        }

    };


    false
}



fn main() {



    let input = load_input();

    println!("Input size: {}", input.len());

    let size = input.iter().filter(|&level| is_safe(level.clone()) ).count();
    println!("Part 1: {}", size);

    let size = input.iter().filter(|&level| is_safe_with_dampener(level.clone()) ).count();

    println!("Part 2: {}", size);


}
