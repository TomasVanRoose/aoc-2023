fn main() {
    let input = include_str!("../input/day3.txt");
    println!("{}", part_one(input));
    println!("{}", part_two(input));
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
                did_find_symbol =
                    check_kernel_for_symbols(&input, |c| c != '.' && !c.is_ascii_digit(), i, width);
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

/*
fn part_two_new(input: &str) -> u32 {
    let width = input.find('\n').unwrap();
    let input: Vec<_> = input.lines().map(|s| s.chars()).flatten().collect();

    let mut current_part_nr = 0;
    let mut digit_count = 0;

    let mut star_indices = Vec::new();

    for (i, c) in input.iter().enumerate() {
        if c.is_ascii_digit() {
            current_part_nr = (current_part_nr * 10) + c.to_digit(10).unwrap();
            digit_count += 1;
        }
        if (!c.is_ascii_digit() || i % width == input.len()) && digit_count > 0 {
            // stel *digit_count == 3* en *index == 4*
            // . 1 2 3 . -> sliding window 0 1 2 3 4
        }
    }

    unimplemented!()
}
*/

fn part_two(input: &str) -> u32 {
    let width = input.find('\n').unwrap();
    let input: Vec<_> = input.lines().map(|s| s.chars()).flatten().collect();

    let mut temp_count = vec![0; input.len()];
    let mut star_count = temp_count.clone();
    let mut part_combo = vec![1; input.len()];

    let mut current_part_nr = 0;
    let mut digit_count = 0;

    for (i, c) in input.iter().enumerate() {
        if c.is_ascii_digit() {
            current_part_nr = (current_part_nr * 10) + c.to_digit(10).unwrap();
            digit_count += 1;

            find_stars(&mut temp_count, &input, i, width);
        }
        if (!c.is_ascii_digit() || i % width == input.len()) && digit_count > 0 {
            for idx in 0..star_count.len() {
                if temp_count[idx] > 0 {
                    star_count[idx] += temp_count[idx];
                    part_combo[idx] *= current_part_nr;
                }
            }
            current_part_nr = 0;
            digit_count = 0;
            temp_count.fill(0);
        }
    }

    return star_count
        .iter()
        .zip(part_combo.iter())
        .filter(|(&c, _)| c == 2)
        .map(|(_, s)| s)
        .sum();
}

fn find_stars(count: &mut [usize], chars: &[char], idx: usize, width: usize) {
    for i in -1..=1 {
        for j in -1..=1 {
            if let Some((c, x)) = usize::try_from(idx as isize + (i * width as isize) + j)
                .ok()
                .and_then(|x| chars.get(x).map(|c| (c, x)))
            {
                if *c == '*' {
                    count[x] = 1;
                }
            }
        }
    }
}

fn check_kernel_for_symbols(
    chars: &[char],
    symbols: fn(char) -> bool,
    idx: usize,
    width: usize,
) -> bool {
    for i in -1..=1 {
        for j in -1..=1 {
            if let Some(c) = usize::try_from(idx as isize + (i * width as isize) + j)
                .ok()
                .and_then(|x| chars.get(x))
            {
                if symbols(*c) {
                    return true;
                }
            }
        }
    }
    false
}
