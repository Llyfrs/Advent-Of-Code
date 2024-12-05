use std::cmp::Ordering;
use std::collections::HashMap;


fn load_input() -> Vec<Update> {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let lines = input.lines();

    let mut updates = vec![];
    let mut hashmap : HashMap<i32, Vec<i32>> = HashMap::new();

    for line in lines {
        if line.contains("|") {
            let mut split = line.split("|");
            let rule = split.next().unwrap().parse().unwrap();
            let value = split.next().unwrap().parse().unwrap();

            let mut rule_list = hashmap.entry(rule).or_insert(vec![]);
            rule_list.push(value);

        }
        // Runs after all the full hashmap is created
        else if line.contains(",") {

            let split = line.split(",");
            let update: Vec<Element> = split.map(|x| {
                let mut element = Element {
                    value: x.parse().unwrap(),
                    ords: hashmap.entry(x.parse().unwrap()).or_insert(vec![]).clone()
                };
                element
            }).collect();

            updates.push(Update {
                update
            });

        }
    }

    updates
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Element {
    value: i32,
    ords: Vec<i32>
}

impl PartialOrd<Self> for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {

        if self.ords.contains(&other.value) {
            return Ordering::Less;
        }

        if other.ords.contains(&self.value) {
            return Ordering::Greater;
        }

        Ordering::Equal
    }
}


#[derive(Debug)]
struct Update {
    update: Vec<Element>
}

impl Update {
    fn new() -> Update {
        Update {
            update: vec![]
        }
    }

    fn print(&self) {
        print!("Update: ");
        for element in self.update.iter() {
            print!("{:?}, ", element.value);
        }
    }

    fn is_sorted(&self) -> bool {
        self.update.windows(2).all(|w| w[0] < w[1])
    }

    fn middle_number(&self) -> i32 {
        let middle = ((self.update.len() - 1) / 2 );
        self.update[middle].value
    }

    fn sort(&mut self) {
        self.update.sort();
    }
}


fn main() {

    let mut updates = load_input();

    let mut count = 0;

    let mut count2 = 0;

    for update in updates.iter_mut() {
        if update.is_sorted() {
            count += update.middle_number();

        } else {
            update.sort();
            count2 += update.middle_number();
        }
    }


    println!("Part 1: {}", count);

    println!("Part 2: {}", count2);
}
