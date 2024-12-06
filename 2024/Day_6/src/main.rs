use std::collections::HashSet;

enum Tile {
    Empty{was_visited: bool},
    Obstacle,
}



fn load_input() -> Map {
    let string = std::fs::read_to_string("src/input.txt").unwrap();

    let mut map = Map::new();

    for line in string.lines(){
        let mut row = Vec::new();
        for c in line.chars(){
            match c {
                '.' => row.push(Tile::Empty{was_visited: false}),
                '#' => row.push(Tile::Obstacle),
                '^' => { // Could be that the player can be > < v as well but neither my input or the example has it like that, so I didn't bother parsing it
                    row.push(Tile::Empty{was_visited: true});
                    let player = Player{x: row.len() as i32 -1, y: map.tiles.len() as i32 , direction: 0};
                    map.player = player;
                }
                _ => panic!("Invalid character in input file"),
            }
        }

        map.tiles.push(row);
    }

    map
}


#[derive(Clone, Copy, Eq, PartialEq)]
#[derive(Hash)]
struct Player {
    x: i32,
    y: i32,
    direction: i8, // 0 = up, 1 = right, 2 = down, 3 = left
}

impl Player {
    fn move_forward(&mut self) {
        let (x, y) = self.looking_to();
        self.x = x;
        self.y = y;
    }

    fn looking_to(&self) -> (i32, i32) {
        match self.direction {
            0 => (self.x, self.y - 1),
            1 => (self.x + 1, self.y),
            2 => (self.x, self.y + 1),
            3 => (self.x - 1, self.y),
            _ => panic!("Invalid direction"),
        }
    }

    fn turn_right(&mut self) {
        self.direction = (self.direction + 1) % 4;
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    player: Player,
    visited: i32,
}

impl Map {
    fn new() -> Map {
        Map {
            tiles: Vec::new(),
            player: Player{x: 0, y: 0, direction: 0},
            visited: 1,
        }
    }

    fn print(&self) {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if self.player.x == x as i32 && self.player.y == y as i32 {
                    match self.player.direction {
                        0 => print!("^"),
                        1 => print!(">"),
                        2 => print!("v"),
                        3 => print!("<"),
                        _ => panic!("Invalid direction"),
                    }
                } else {
                    match tile {
                        Tile::Empty{was_visited} => print!("{}", if *was_visited {"X"} else {"."}),
                        Tile::Obstacle => print!("#"),
                    }
                }
            }
            println!();
        }

    }

    fn is_in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && y < self.tiles.len() as i32 && x < self.tiles[0].len() as i32
    }

    fn next_step(&mut self) -> bool {
        let (x, y) = self.player.looking_to();

        if !self.is_in_bounds(x, y) {
            return false;
        }

        match self.tiles[y as usize][x as usize] {
            Tile::Empty{was_visited} => {
                self.tiles[y as usize][x as usize] = Tile::Empty{was_visited: true};
                self.visited += !was_visited as i32;
                self.player.move_forward();
            },
            Tile::Obstacle => {
                self.player.turn_right();
            },
        };

        true
    }

    fn has_loop(&mut self) -> bool {
        let mut player_history: HashSet<_> = HashSet::new();
        let player_start = self.player;

        player_history.insert(self.player);

        while self.next_step() {
            if !player_history.insert(self.player) {
                // If the insert fails, the player has already been visited
                self.player = player_start;
                return true;
            }
        }

        self.player = player_start;
        false
    }

    fn set_tile(&mut self, x: i32, y: i32, tile: Tile) {

        if self.player.x == x && self.player.y == y {
            println!("Can't set player tile");
            return;
        }

        if !self.is_in_bounds(x, y) {
            return;
        }

        self.tiles[y as usize][x as usize] = tile;
    }

    fn get_visited_tiles(&self) -> Vec<(i32, i32)> {
        let mut visited = Vec::new();
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                match tile {
                    Tile::Empty{was_visited} => {
                        if *was_visited {
                            visited.push((x as i32, y as i32));
                        }
                    },
                    Tile::Obstacle => {},
                }
            }
        }
        visited
    }
}


fn main() {

    let mut map = load_input();

    while map.next_step() {
    }

    println!("Part 1: {}", map.visited);

    let mut count = 0;

    let mut map_testing = load_input();

    let visited = map.get_visited_tiles();

    for (x, y) in visited {

        map_testing.set_tile(x, y, Tile::Obstacle);
        if map_testing.has_loop() {
            count += 1;
        }

        map_testing.set_tile(x, y, Tile::Empty{was_visited: false});
    }

    println!("Part 2: {}", count);
}
