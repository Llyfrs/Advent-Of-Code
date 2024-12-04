use std::collections::HashMap;

fn load_input() -> Vec<Vec<char>> {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    input.lines().map(|line| line.chars().collect()).collect()
}

fn search_line(line: &Vec<char>) -> i32 {
    let mut count = 0;

    if line.len() < 4 {
        return 0
    }

    for i in 0..line.len()-3 {

        if line[i] == 'X' && line[i+1] == 'M' && line[i+2] == 'A' && line[i+3] == 'S' {
            count += 1
        }

        if line[i] == 'S' && line[i+1] == 'A' && line[i+2] == 'M' && line[i+3] == 'X' {
            count += 1
        }
    }

    count
}

fn generate_main_diagonals(grid : &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let (len_c, len_r) = (grid[0].len() as i32, grid.len() as i32);

    let mut diagonals: HashMap<i32, Vec<char>> = HashMap::new();
    let mut diagonals_alt : HashMap<i32, Vec<char>> = HashMap::new();

    for i in 0..len_r {
        for j in 0..len_c {
            let diff = i-j;
            if diagonals.contains_key(&diff) {
                diagonals.get_mut(&diff).unwrap().push(grid[i as usize][j as usize])
            } else {
                diagonals.insert(diff, vec![grid[i as usize][j as usize]]);
            }

            let sum = i + j;

            if diagonals_alt.contains_key(&sum) {
                diagonals_alt.get_mut(&sum).unwrap().push(grid[i as usize][j as usize])
            } else {
                diagonals_alt.insert(sum, vec![grid[i as usize][j as usize]]);
            }
        }
    }

    let mut results = Vec::new();

    for dig in diagonals.values(){
        results.push(dig.clone());
    }

    for dig in diagonals_alt.values(){
        results.push(dig.clone());
    }

    results
}


fn search_x_mas(grid : &Vec<Vec<char>>) -> i32 {
    let (len_c, len_r) = (grid[0].len(), grid.len());

    let mut count = 0;

    for i in 0..len_r - 2 {
        for j in 0..len_c- 2 {

            let vector1 : String = vec![
                grid[i][j], grid[i+1][j+1], grid[i+2][j+2]
            ].into_iter().collect();

            let vector2 : String = vec![
                grid[i][j+2], grid[i+1][j+1], grid[i+2][j]
            ].into_iter().collect();

            if (vector2 == "MAS" || vector2 == "SAM") && (vector1 == "MAS" || vector1 == "SAM") {
                count += 1;
            }
        }


    }
    count
}






fn main() {
    let mut sum = 0;

    let input = load_input();
    for line in &input {
        sum += search_line(line)
    };

    let len = input.get(0).unwrap().len();

    for i in 0..len {
        let mut temp : Vec<char> = Vec::new();
        for line in &input {
            temp.push(line[i])
        }
        sum += search_line(&temp)
    }

    let diagonals = generate_main_diagonals(&input);

    for line in diagonals {
        sum += search_line(&line)
    }

    println!("Part 1: {sum}");

    sum = search_x_mas(&input);

    println!("Part 2: {sum}");
}
