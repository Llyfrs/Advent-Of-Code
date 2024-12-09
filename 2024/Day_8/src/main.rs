use std::collections::{HashMap, HashSet};

struct Antenna {
    x: i32,
    y: i32,
}

impl Antenna {
    fn vector_to(&self, other: &Antenna) -> (i32, i32) {
        (other.x - self.x, other.y - self.y)
    }
}
struct Grid {
    antennas: HashMap<char, Vec<Antenna>>,
    width: i32,
    height: i32,
}

impl Grid {
    fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.width && y >= 0 && y < self.height
    }


    // Part 1
    fn get_antinode(&self) -> HashSet<(i32, i32)> {
        let mut antinode = HashSet::new();

        for frequency in self.antennas.keys() {
            let antennas = &self.antennas[frequency];

            for i in 0..antennas.len() {
                for j in i+1..antennas.len() {

                    let antenna1 = &antennas[i];
                    let antenna2 = &antennas[j];

                    let (dx, dy) = antenna1.vector_to(antenna2);

                    if self.is_in_bounds(antenna1.x - dx, antenna1.y - dy) {
                        antinode.insert((antenna1.x - dx, antenna1.y - dy));
                    }

                    if self.is_in_bounds(antenna2.x + dx, antenna2.y + dy) {
                        antinode.insert((antenna2.x + dx, antenna2.y + dy));
                    }
                }
            }

        }

        antinode
    }


    // Part 2

    // This was bit stupid you are supposed find all point on the line,
    // but what you are really doint is just adding the distance over and over gain
    // which is not the same thing
    fn get_antinode_fixed(&self) -> HashSet<(i32, i32)> {
        let mut antinode = HashSet::new();

        for frequency in self.antennas.keys() {
            let antennas = &self.antennas[frequency];

            for i in 0..antennas.len() {
                for j in i + 1..antennas.len() {
                    let antenna1 = &antennas[i];
                    let antenna2 = &antennas[j];

                    let (dx, dy) = antenna1.vector_to(antenna2);


                    for direction in [-1, 1] {
                        let mut x = antenna1.x;
                        let mut y = antenna1.y;

                        while self.is_in_bounds(x as i32, y as i32) {

                            antinode.insert((x as i32, y as i32));

                            x += dx * direction;
                            y += dy * direction;
                        }
                    }
                }
            }
        }

        antinode
    }

}

fn normalize(v: (i32, i32)) -> (f32, f32) {
    let magnitude = ((v.0 * v.0 + v.1 * v.1) as f32).sqrt();
    if magnitude == 0.0 {
        return (0.0, 0.0);
    }
    (v.0 as f32 / magnitude, v.1 as f32 / magnitude)
}

fn load_input() -> Grid {
    let content = std::fs::read_to_string("src/input.txt").unwrap();
    let mut antennas = HashMap::new();

    let lines: Vec<&str> = content.lines().collect();  // Collect lines into a Vec

    let width = lines[0].len() as i32;
    let height = lines.len() as i32;

    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                let frequency = c;
                let antenna = Antenna {
                    x: x as i32,
                    y: y as i32,
                };

                antennas.entry(frequency).or_insert(vec![]).push(antenna);
            }
        }
    }

    Grid {
        antennas,
        width,
        height,
    }
}

fn main() {
    let grid = load_input();

    println!("Part 1: {:?}", grid.get_antinode().len());

    println!("Part 2: {:?}", grid.get_antinode_fixed().len());

}
