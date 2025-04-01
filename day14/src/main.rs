use common::read_lines;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Block {
    Air = 0,
    Wall = 1,
    Sand = 2,
}

struct Cave {
    width: usize,
    grid: Vec<u8>,
}

impl Cave {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            grid: vec![Block::Air as u8; width * height],
        }
    }

    fn set(&mut self, x: usize, y: usize, block: Block) {
        self.grid[y * self.width + x] = block as u8;
    }

    fn get(&self, x: usize, y: usize) -> Block {
        match self.grid[y * self.width + x] {
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
    let mut cave = Cave::new(1000, 1000);

    parse_input(lines, &mut cave);
    print_cave(&cave, 475, 550, 0, 165);
}

fn print_cave(cave: &Cave, x_min: usize, x_max: usize, y_min: usize, y_max: usize) {
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            match cave.get(x, y) {
                Block::Air => print!("."),
                Block::Wall => print!("#"),
                Block::Sand => print!("o"),
            }
        }
        println!();
    }
}

fn parse_input(lines: Vec<String>, cave: &mut Cave) {
    let walls: Vec<Vec<(u16, u16)>> = lines
        .iter()
        .map(|line| {
            line.split(" -> ")
                .map(|coords| {
                    let (first, last) = coords.split_once(",").unwrap();
                    let x = first.parse::<u16>().expect("Invalid X coordinate.");
                    let y = last.parse::<u16>().expect("Invalid X coordinate.");

                    (x, y)
                })
                .collect::<Vec<(u16, u16)>>()
        })
        .collect();

    println!("{:?}", walls);
    println!();

    walls.iter().for_each(|wall| {
        for i in 1..wall.len() {
            let (x1, y1) = wall[i - 1];
            let (x2, y2) = wall[i];

            if y1 == y2 {
                let x_min = x1.min(x2) as usize;
                let x_max = x1.max(x2) as usize;
                let y = y1 as usize;

                cave.grid[(y * cave.width + x_min)..(y * cave.width + x_max + 1)]
                    .fill(Block::Wall as u8);
            }

            if x1 == x2 {
                let y_min = y1.min(y2) as usize;
                let y_max = y1.max(y2) as usize;

                for i in y_min..=y_max {
                    cave.set(x1 as usize, i, Block::Wall);
                }
            }
        }
    });
}
