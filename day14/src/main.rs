use common::read_lines;

#[derive(PartialEq, Eq)]
enum Block {
    Air = 0,
    Wall = 1,
    Sand = 2,
}

#[derive(Debug, Clone)]
struct Coords {
    x: usize,
    y: usize,
}

struct Cave {
    size: usize,
    grid: Vec<u8>,
}

impl Cave {
    fn new(size: usize) -> Self {
        Self {
            size,
            grid: vec![Block::Air as u8; size * size],
        }
    }

    fn set(&mut self, x: usize, y: usize, block: Block) {
        self.grid[y * self.size + x] = block as u8;
    }

    fn get(&self, x: usize, y: usize) -> Block {
        match self.grid[y * self.size + x] {
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
    let mut cave = Cave::new(1000);

    parse_input(lines, &mut cave);

    let highest_y: usize = (cave.grid.iter().rposition(|&x| x == 1).unwrap() / cave.size) + 2;
    println!("Highest y: {}", highest_y);

    let count = pour_sand(&mut cave, highest_y);
    println!("Units: {}", count);

    print_cave(&cave, 400, 600, 0, highest_y);
}

fn pour_sand(cave: &mut Cave, highest_y: usize) -> usize {
    let mut count = 0;
    let starting_pos = Coords { x: 500, y: 0 };
    let mut coords: Coords = starting_pos.clone();

    while coords.y < cave.size - 1 {
        if let Some(next_coords) = move_sand(&coords, cave, highest_y) {
            coords = next_coords;
        } else {
            cave.set(coords.x, coords.y, Block::Sand);
            count += 1;

            if coords.y == 0 {
                break;
            }

            coords = starting_pos.clone();
        }
    }

    count
}

fn move_sand(coords: &Coords, cave: &Cave, highest_y: usize) -> Option<Coords> {
    if coords.y + 1 == highest_y {
        return None;
    }

    if cave.get(coords.x, coords.y + 1) == Block::Air {
        return Some(Coords {
            x: coords.x,
            y: coords.y + 1,
        });
    } else {
        // Attempt to move diagonally (left)
        if cave.get(coords.x - 1, coords.y + 1) == Block::Air {
            return Some(Coords {
                x: coords.x - 1,
                y: coords.y + 1,
            });
        }

        // Attempt to move diagonally (right)
        if cave.get(coords.x + 1, coords.y + 1) == Block::Air {
            return Some(Coords {
                x: coords.x + 1,
                y: coords.y + 1,
            });
        }
    }

    None
}

fn parse_input(lines: Vec<String>, cave: &mut Cave) {
    let walls: Vec<Vec<Coords>> = lines
        .iter()
        .map(|line| {
            line.split(" -> ")
                .map(|coords| {
                    let (first, last) = coords.split_once(",").unwrap();
                    let x = first.parse::<usize>().expect("Invalid X coordinate.");
                    let y = last.parse::<usize>().expect("Invalid X coordinate.");

                    Coords { x, y }
                })
                .collect::<Vec<Coords>>()
        })
        .collect();

    walls.iter().for_each(|wall| {
        for i in 1..wall.len() {
            let left = &wall[i - 1];
            let right = &wall[i];

            if left.y == right.y {
                let x_min = left.x.min(right.x);
                let x_max = left.x.max(right.x);

                cave.grid[(left.y * cave.size + x_min)..(left.y * cave.size + x_max + 1)]
                    .fill(Block::Wall as u8);
            }

            if left.x == right.x {
                let y_min = left.y.min(right.y);
                let y_max = left.y.max(right.y);

                for i in y_min..=y_max {
                    cave.set(left.x, i, Block::Wall);
                }
            }
        }
    });
}

fn print_cave(cave: &Cave, x_min: usize, x_max: usize, y_min: usize, y_max: usize) {
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if y == y_max {
                print!("#");
                continue;
            }

            match cave.get(x, y) {
                Block::Air => print!("."),
                Block::Wall => print!("#"),
                Block::Sand => print!("o"),
            }
        }
        println!();
    }
}
