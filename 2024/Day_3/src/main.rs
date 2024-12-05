use regex::Regex;


fn load_input() -> String {
    std::fs::read_to_string("src/input.txt").unwrap()
}

fn main() {

    let input = load_input();
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;
    for cap in re.captures_iter(&input) {
        let a = cap[1].parse::<i32>().unwrap();
        let b = cap[2].parse::<i32>().unwrap();
        sum += a * b;
    }

    println!("Part 1: {}", sum);


    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut sum = 0;
    let mut enabled = true;

    for cap in re.captures_iter(&input) {
        if &cap[0] == "do()" {
            enabled = true;
        } else if &cap[0] == "don't()" {
            enabled = false;
        } else if enabled {
            let a = cap[1].parse::<i32>().unwrap();
            let b = cap[2].parse::<i32>().unwrap();
            sum += a * b;
        }
    }

    println!("Part 2: {}", sum);
}
