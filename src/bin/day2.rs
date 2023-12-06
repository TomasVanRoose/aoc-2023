fn main() {
    let input = include_str!("../input/day2.txt");
    println!("{}", part_one(input));
    println!("{}", part_two(input));
}

fn part_one(input: &str) -> usize {
    // 12 red, 13 green, 14 blue
    input
        .lines()
        .map(|line| {
            // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            let (game_id, game_str) = line.split_once(':').expect("No : in string");
            let game_id = parse_game_id(game_id);

            if game_str
                .split(';')
                .flat_map(|turn| {
                    // 1 blue, 2 green
                    turn.split(',').map(|marbles| {
                        // 12 blue
                        let (digit, color) = marbles
                            .trim_start()
                            .split_once(' ')
                            .expect("no space in number");
                        let digit = digit.parse::<usize>().unwrap();
                        match color {
                            "red" => 12 >= digit,
                            "green" => 13 >= digit,
                            "blue" | _ => 14 >= digit,
                        }
                    })
                })
                .all(|x| x)
            {
                game_id
            } else {
                0
            }
        })
        .sum()
}

fn part_two(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            let (_, game_str) = line.split_once(':').expect("No : in string");

            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;

            for (digit, color) in game_str.split(';').flat_map(|turn| {
                turn.split(',').map(|marbles| {
                    let (digit, color) = marbles
                        .trim_start()
                        .split_once(' ')
                        .expect("no space in number");
                    (digit.parse::<usize>().unwrap(), color)
                })
            }) {
                match color {
                    "red" => max_red = std::cmp::max(max_red, digit),
                    "green" => max_green = std::cmp::max(max_green, digit),
                    "blue" | _ => max_blue = std::cmp::max(max_blue, digit),
                }
            }
            return max_red * max_blue * max_green;
        })
        .sum()
}

fn parse_game_id(input: &str) -> usize {
    input
        .trim_start_matches("Game ")
        .parse()
        .expect("No id found")
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(INPUT), 8);
    }

    #[test]
    fn test_parse_game_id() {
        assert_eq!(parse_game_id("Game 1"), 1);
        assert_eq!(parse_game_id("Game 11"), 11);
    }
}
