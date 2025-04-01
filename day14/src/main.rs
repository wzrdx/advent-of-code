use common::read_lines;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Block {
    Air = 0,
    Wall = 1,
    Sand = 2,
}

struct Cave {
    width: usize,
    // height: usize,
    data: Vec<u8>,
}

impl Cave {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            data: vec![Block::Air as u8; width * height],
        }
    }

    fn set(&mut self, x: usize, y: usize, block: Block) {
        self.data[y * self.width + x] = block as u8;
    }

    fn get(&self, x: usize, y: usize) -> Block {
        match self.data[y * self.width + x] {
            0 => Block::Air,
            1 => Block::Wall,
            2 => Block::Sand,
            _ => panic!("Invalid block type."),
        }
    }
}

// Day #14
pub fn main() {
    let lines: Vec<String> = read_lines("day14/input.txt");
    let mut cave = Cave::new(5, 5);
    cave.set(0, 0, Block::Wall);

    println!("{:?}", lines);
    println!("{:?}", cave.data);
}
