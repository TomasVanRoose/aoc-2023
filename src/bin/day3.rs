fn main() {
    let input = include_str!("../input/day3.txt");
    println!("{}", part_one(input));
}

fn part_one(input: &str) -> u32 {
    let width = input.find('\n').unwrap();
    let input: Vec<_> = input.lines().map(|s| s.chars()).flatten().collect();

    let mut current_part_nr = 0;
    let mut digit_count = 0;
    let mut did_find_symbol = false;

    let mut sum = 0;

    for (i, c) in input.iter().enumerate() {
        if c.is_ascii_digit() {
            if i % width == 0 {
                if did_find_symbol {
                    sum += current_part_nr;
                }
                current_part_nr = 0;
                digit_count = 0;
                did_find_symbol = false;
            }

            current_part_nr = (current_part_nr * 10) + c.to_digit(10).unwrap();
            digit_count += 1;
            if !did_find_symbol {
                did_find_symbol = check_kernel_for_symbols(&input, i, width);
            }
        } else if digit_count > 0 {
            if did_find_symbol {
                sum += current_part_nr;
            }
            current_part_nr = 0;
            digit_count = 0;
            did_find_symbol = false;
        }
    }

    return sum;
}

fn check_kernel_for_symbols(chars: &[char], idx: usize, width: usize) -> bool {
    for i in -1..=1 {
        for j in -1..=1 {
            if let Some(c) = usize::try_from(idx as isize + (i * width as isize) + j)
                .ok()
                .and_then(|x| chars.get(x))
            {
                if *c != '.' && !c.is_ascii_digit() {
                    return true;
                }
            }
        }
    }
    false
}

pub mod matrix {

    pub struct Matrix<'a, T> {
        inner: &'a [T],
        width: usize,
        height: usize,
    }

    impl<'a, T> Matrix<'a, T> {
        pub fn new(inner: &'a [T], width: usize, height: usize) -> Self {
            Self {
                inner,
                width,
                height,
            }
        }

        pub fn window(&self) {}
    }
}
