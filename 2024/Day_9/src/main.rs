use std::path::Iter;

struct Segment {
    size: u32,
    data: Vec<i32>,
}

impl Segment {
    fn print(&self) {
        for i in 0..self.size {
            if i < self.data.len() as u32 {
                print!("{}", self.data[i as usize]);
            } else {
                print!(".");
            }
        }
    }

    fn is_full(&self) -> bool {
        self.data.len() as u32 == self.size
    }

    fn is_empty(&self) -> bool {
        self.data.len() as u32 == 0
    }

    fn add(&mut self, data: i32) {
        if !self.is_full() {
            self.data.push(data);
        }
    }
}

struct Memory {
    segments: Vec<Segment>
}

impl Memory {
    fn print(&self) {
        for segment in &self.segments {
            segment.print();
        }
        println!();
    }

    fn push(&mut self, data: i32) -> (i32, i32) {
        for (i, segment) in &mut self.segments.iter_mut().enumerate() {
            if !segment.is_full() {
                segment.add(data);
                return (i as i32, segment.data.len() as i32);
            }
        }
        return (-1, -1);
    }


    // get last element from the last non-empty segment
    fn pop(&mut self) -> i32 {
        for segment in self.segments.iter_mut().rev() {
            if !segment.is_empty() {
                return segment.data.pop().unwrap();
            }
        }
        return -1;
    }

    fn defragment(&mut self) {
        let mut last_affected = (-1, -1);

        loop {

            let pop = self.pop();
            let (segment_id, segment_size) = self.push(pop);
            if segment_id == last_affected.0 && segment_size == last_affected.1 {
                break;
            }

            last_affected = (segment_id, segment_size);
        }

    }

}

// TO lazy
/*impl Iterator for Memory {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {

    }
}*/


fn load_input() -> Memory {
    let string = std::fs::read_to_string("src/input.txt").unwrap();

    let mut empty = false;

    let mut memory: Vec<Segment> = Vec::new();
    let mut id = 0;

    for char in string.chars() {
        if empty {
            memory.push(Segment {
                size: char.to_digit(10).unwrap(),
                data: Vec::new(),
            });

            empty = false;

        } else {
            memory.push(Segment {
                size: char.to_digit(10).unwrap(),
                data: vec![id; char.to_digit(10).unwrap() as i32 as usize],
            });

            empty = true;
            id += 1;
        }
    };

    Memory {
        segments: memory,
    }
}

fn main() {

    let mut memory = load_input();

    //memory.print();



    memory.defragment();

    // memory.print();

    let mut sum : u64 = 0;
    let mut off = 0;
    for (i, segment) in memory.segments.iter().enumerate() {
        for (j, value) in segment.data.iter().enumerate() {

            sum += (off + j) as u64 * (*value as u64)

        }

        off += segment.data.len()
    }

    println!("Part 1: {sum}")


}
