fn main() {
    let input = include_str!("../input/day5.txt");
    println!("{}", part1(input));
    println!("{}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut line_iter = input.lines();

    let mut seeds: Vec<_> = line_iter
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .filter_map(|s| usize::from_str(s).ok())
        .collect();

    let _ = line_iter.next();
    loop {
        if line_iter.next().is_none() {
            break;
        }

        let mut mappings = Vec::new();
        for line in line_iter.by_ref() {
            if line.is_empty() {
                break;
            }
            mappings.push(Mapping::from_str(line).unwrap());
        }

        for seed in seeds.iter_mut() {
            *seed = mappings
                .iter()
                .find_map(|mapping| mapping.map(*seed))
                .unwrap_or(*seed);
        }
    }
    seeds
        .into_iter()
        .map(|x| u32::try_from(x).unwrap())
        .min()
        .unwrap()
}

fn part2(input: &str) -> u32 {
    unimplemented!()
}

use std::ops::Range;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct Mapping {
    source: Range<usize>,
    destination: Range<usize>,
}

impl Mapping {
    fn map(&self, value: usize) -> Option<usize> {
        if self.source.contains(&value) {
            Some(self.destination.start + (value - self.source.start))
        } else {
            None
        }
    }
}

impl FromStr for Mapping {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num_iter = s.split(' ');

        let destination_start = usize::from_str(num_iter.next().expect("Should contain 3 nums"))?;
        let source_start = usize::from_str(num_iter.next().expect("Should contain 3 nums"))?;
        let length = usize::from_str(num_iter.next().expect("Should contain 3 nums"))?;

        Ok(Self {
            source: source_start..source_start + length,
            destination: destination_start..destination_start + length,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_mapping() {
        assert_eq!(
            Mapping::from_str("50 98 2").unwrap(),
            Mapping {
                source: 98..100,
                destination: 50..52
            }
        );
    }
    #[test]
    fn test_2_mapping() {
        assert_eq!(
            Mapping::from_str("52 50 48").unwrap(),
            Mapping {
                source: 50..98,
                destination: 52..100
            }
        );
    }

    #[test]
    fn test_mapping() {
        let mapping = Mapping::from_str("50 98 2").unwrap();
        assert_eq!(mapping.map(97), None);
        assert_eq!(mapping.map(98), Some(50));
        assert_eq!(mapping.map(99), Some(51));
        assert_eq!(mapping.map(100), None);
    }

    const INPUT: &str = r"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 35);
    }

    #[test]
    fn test_part2() {}
}
