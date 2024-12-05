use std::collections::HashMap;

fn load_input() -> (RuleSet, Vec<Update>) {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    let lines = input.lines();

    let mut rules = RuleSet::new();
    let mut updates = vec![];

    for line in lines {
        if line.contains("|") {
            let mut split = line.split("|");
            let rule = split.next().unwrap().parse().unwrap();
            let value = split.next().unwrap().parse().unwrap();
            rules.add_rule(value, rule);

        } else if line.contains(",") {

            let mut update = Update::new();

            let array : Vec<i32> = line.split(',')
                .map(|x| x.parse()
                    .unwrap()).collect();

            update.set_update(array);
            updates.push(update);

        }
    }

    (rules, updates)
}

struct RuleSet {
    rules: HashMap<i32, Vec<i32>>
}

impl RuleSet {
    fn new() -> RuleSet {
        RuleSet {
            rules: HashMap::new()
        }
    }

    fn add_rule(&mut self, value: i32, rule: i32 ) {
        let mut rule_list = self.rules.entry(rule).or_insert(vec![]);
        rule_list.push(value);
    }

    fn get_rule(&self, rule: i32) -> Vec<i32> {
        match self.rules.get(&rule) {
            Some(x) => x.clone(),
            None => vec![]
        }
    }
}

struct Update {
    update: Vec<i32>
}

impl Update {
    fn new() -> Update {
        Update {
            update: vec![]
        }
    }

    fn set_update(&mut self, update: Vec<i32>) {
        self.update = update;
    }

    fn check_update(&self, rule_set: &RuleSet) -> bool {
        let mut visited = vec![];
        for update in self.update.iter() {
            let rule = rule_set.get_rule(*update);

            if rule.iter().any(|x| visited.contains(x)) {
                return false;
            }
            visited.push(*update);
        }

        true
    }

    fn fix(&mut self, rule_set: &RuleSet) {
        let mut visited = vec![];
        let mut wrong = vec![];

        for update in self.update.iter() {
            let rule = rule_set.get_rule(*update);
            if rule.iter().any(|x| visited.contains(x)) {
                wrong.push(*update);
            } else {
                visited.push(*update);
            }
        }

        println!("Update {:?}", &self.update);

        println!("Wrong: {:?}", &wrong);
        println!("Visited {:?}", &visited);


        let mut new = vec![];

        for i in (0..visited.len()).rev() {
            for &numb in &wrong {
                let upper_cut = visited[..i].to_vec();
                let rule = rule_set.get_rule(visited[i]);

                if rule.iter().any(|x| !upper_cut.contains(x)) {
                    new.insert(0, numb);
                }
            }

            new.insert(0, visited[i]);
            wrong.retain(|&x| !new.contains(&x));
        }


        println!("fixed ? {:?}", &new);

        self.update = new;

    }

    fn middle_number(&self) -> i32 {
        let middle = ((self.update.len() - 1) / 2 );
        self.update[middle]
    }
}





fn main() {

    let (rules, mut update) = load_input();
    let mut count = 0;
    let mut count2 = 0;
    for u in update.iter_mut() {
        if u.check_update(&rules) {
            count += u.middle_number();
        } else {
            while !u.check_update(&rules) {
                u.fix(&rules);
            }
            count2 += u.middle_number();
        }
    }

    println!("Part 1: {}", count);

    println!("Part 2: {}", count2);
}
