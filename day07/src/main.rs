use std::iter::Peekable;

// Day #7
pub fn main() {
    let bytes = include_bytes!("../input.txt");
    let mut sum = 0;

    read(&mut bytes.split(|b| b == &b'\n').peekable(), &mut sum);
    println!("{}", sum);
}

fn read(lines: &mut Peekable<impl Iterator<Item = &'static [u8]>>, sum: &mut u64) -> u64 {
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
                let sum_i = read(lines, sum);
                size += sum_i
            }
        }
    }

    // Prints every dir's size, including the root dir which contains the '/' dir
    println!("Directory size {:?}", size);

    if size <= 100_000 {
        *sum += size;
    }

    size
}
