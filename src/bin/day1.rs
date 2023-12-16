fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let input = include_str!("../input/day1.txt");
    let result: u32 = input
        .lines()
        .map(|line| {
            let first = line.chars().find_map(|c| c.to_digit(10)).unwrap();
            let last = line.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
            (first * 10) + last
        })
        .sum();
    println!("{}", result);
}

fn find_first(line: &str) -> u32 {
    let digit_str = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digit_idx = line.find(|c: char| c.is_ascii_digit()).unwrap();
    let digit = line.chars().nth(digit_idx).unwrap().to_digit(10).unwrap();

    if let Some((digit_str_idx, digit_str_pos)) = digit_str
        .iter()
        .enumerate()
        .filter_map(|(i, ds)| line.find(ds).map(|ds_pos| (i, ds_pos)))
        .min_by_key(|&(_, ds_pos)| ds_pos)
    {
        if digit_str_pos < digit_idx {
            return digit_str_idx as u32 + 1;
        }
    }
    return digit;
}

fn find_last(line: &str) -> u32 {
    let digit_str = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digit_idx = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
    let digit = line.chars().nth(digit_idx).unwrap().to_digit(10).unwrap();

    if let Some((digit_str_idx, digit_str_pos)) = digit_str
        .iter()
        .enumerate()
        .filter_map(|(i, ds)| line.rfind(ds).map(|ds_pos| (i, ds_pos)))
        .max_by_key(|&(_, ds_pos)| ds_pos)
    {
        if digit_str_pos > digit_idx {
            return digit_str_idx as u32 + 1;
        }
    }
    return digit;
}

fn part_two() {
    let input = include_str!("../input/day1.txt");
    let result: u32 = input
        .lines()
        .map(|line| {
            let first = find_first(&line);
            let last = find_last(&line);
            (first * 10) + last
        })
        .sum();
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(find_first("two1nine"), 2);
        assert_eq!(find_first("eightwo3three"), 8);
        assert_eq!(find_first("abcone2threexyz"), 1);
        assert_eq!(find_first("xtwone3four"), 2);
        assert_eq!(find_first("4nineeightseven2"), 4);
    }

    #[test]
    fn test_last() {
        assert_eq!(find_last("two1nine"), 9);
        assert_eq!(find_last("eightwo3three"), 3);
        assert_eq!(find_last("abcone2threexyz"), 3);
        assert_eq!(find_last("xtwone3four"), 4);
        assert_eq!(find_last("4nineeightseven2"), 2);
    }
}
