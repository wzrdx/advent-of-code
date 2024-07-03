use std::iter::Peekable;

// Day #7
pub fn main() {
    let bytes = include_bytes!("../input.txt");
    let mut sum = 0;
    let mut directory_sizes = Vec::new(); // Array to store directory sizes

    read(
        &mut bytes.split(|b| b == &b'\n').peekable(),
        &mut sum,
        &mut directory_sizes,
    );

    println!("{:?}", sum);

    let used_space = directory_sizes.pop().unwrap_or_default();
    let limit: u64 = used_space - 40_000_000;

    let mut best_dir: u64 = used_space;

    for &size in directory_sizes.iter() {
        if (size >= limit) && (size < best_dir) {
            best_dir = size;
        }
    }

    println!("{:?}", best_dir);
}

fn read(
    lines: &mut Peekable<impl Iterator<Item = &'static [u8]>>,
    sum: &mut u64,
    directory_sizes: &mut Vec<u64>,
) -> u64 {
    let mut size = 0;

    while let Some(i) = lines.next() {
        match i {
            b"$ cd .." => {
                break;
            }
            _ if &i[0..3] == b"$ l" => {
                size = std::iter::from_fn(|| lines.next_if(|i| i[0] != b'$'))
                    .filter(|i| i[0] != b'd')
                    .filter_map(|i| atoi::atoi::<u64>(i.split(|b| b == &b' ').next().unwrap()))
                    .sum();
            }
            _ => {
                let sum_i = read(lines, sum, directory_sizes);
                size += sum_i
            }
        }
    }

    // Prints every dir's size, including the root dir which contains the '/' dir
    // println!("Directory size {:?}", size);
    directory_sizes.push(size);

    if size <= 100_000 {
        *sum += size;
    }

    size
}
