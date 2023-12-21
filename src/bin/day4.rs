use std::collections::HashSet;

fn main() {
    let input = include_str!("../input/day4.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, game) = line.split_once(':').expect("Line must have a :");

            let mut number_iter = game.split(' ');
            let winning_numbers = number_iter
                .by_ref()
                .take_while(|&s| s != "|")
                .filter_map(|s| s.parse::<u32>().ok())
                .collect::<HashSet<_>>();

            number_iter
                .filter_map(|s| s.parse::<u32>().ok())
                .filter(|n| winning_numbers.contains(n))
                .fold(0, |acc, _| if acc == 0 { 1 } else { acc * 2 })
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut copies = vec![1; 1];
    for (i, line) in input.lines().enumerate() {
        if copies.len() < i + 1 {
            copies.push(1);
        }
        let (_, game) = line.split_once(':').expect("Line must have a :");

        let mut number_iter = game.split(' ');
        let winning_numbers = number_iter
            .by_ref()
            .take_while(|&s| s != "|")
            .filter_map(|s| s.parse::<u32>().ok())
            .collect::<HashSet<_>>();

        let count = number_iter
            .filter_map(|s| s.parse::<u32>().ok())
            .filter(|n| winning_numbers.contains(n))
            .count();

        for c in 1..=count {
            if copies.len() < i + c + 1 {
                copies.push(1);
            }
            copies[i + c] += copies[i];
        }
    }
    copies.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 30);
    }
}
