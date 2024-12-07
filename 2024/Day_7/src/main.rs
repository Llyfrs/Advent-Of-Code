
use regex::Regex;


enum Operation {
    Add,
    Multiply,
    Combine
}

fn calculate(op: &Operation, a: u64, b: u64) -> u64 {
    match op {
        Operation::Add => a + b,
        Operation::Multiply => a * b,
        Operation::Combine => format!("{}{}", a, b).parse::<u64>().unwrap()
    }
}

struct Test {
    test_value: u64,
    values : Vec<u64>,
    operators: Vec<Operation>
}

fn load_input() -> Vec<Test> {
    let string = std::fs::read_to_string("src/input.txt").unwrap();

    let mut tests = Vec::new();
    let re = Regex::new(r"(\d+)").unwrap();

    for line in string.lines() {
        let numbers: Vec<_> = re.captures_iter(line).map(|x| x[1].parse::<u64>().unwrap()).collect();
        tests.push(Test { test_value: numbers[0], values: numbers[1..].to_vec(), operators: vec![]});
    }
    tests
}

impl Test {


    fn is_valid(&self) -> bool {
        self.test(self.values[0], self.values[1..].to_vec())
    }

    fn test(&self, acc: u64, left : Vec<u64>) -> bool {
        if left.len() == 0 {
            return acc == self.test_value
        }

        if acc > self.test_value {
            return false
        }

        for op in &self.operators {
            if self.test(calculate(op, acc, left[0]), left[1..].to_vec()) {
                return true
            }
        }
        false
    }

}


fn main() {
    let tests = load_input();

    let now = std::time::Instant::now();
    let mut sum = 0;
    for mut test in tests {
        test.operators = vec![Operation::Multiply, Operation::Add];
        if test.is_valid(){
            sum += test.test_value;
        }
    }

    println!("Time: {:?}", now.elapsed());
    println!("Part 1: {sum}");

    sum = 0;

    let now = std::time::Instant::now();

    let tests = load_input();
    for mut test in tests {
        test.operators = vec![Operation::Multiply,Operation::Combine,Operation::Add];
        if test.is_valid() {
            sum += test.test_value;
        }
    }

    println!("Time: {:?}", now.elapsed());
    println!("Part 2: {sum}")

}
