use std::str::FromStr;
use regex::Regex;

pub fn day2() -> std::io::Result<()> {
    let content = std::fs::read_to_string("inputs/input2.txt")
        .expect("Unable to open file");

    let sum = part1(&content);
    let powers = part2(&content);
    println!("Part 1: Game Id sum: {sum}");
    println!("Part 2: Sum of powers: {powers}");

    Ok(())
}

fn part1(lines: &str) -> u32 {
    return lines.lines()
        .filter(|l| line_is_within_bounds(l, 12, 13, 14))
        .map(|l| get_game_number(l))
        .sum();
}

fn part2(lines: &str) -> u32 {
    return lines.lines()
        .map(|l| calculate_power_of_game(l))
        .sum();
}

fn get_game_number(line: &str) -> u32 {
    let re = Regex::new(r"^Game (\d+): .*").unwrap();
    let caps = re.captures(line).unwrap();
    return caps.get(1).map_or(0, |m| FromStr::from_str(m.as_str()).unwrap());
}

fn calculate_power_of_game(line: &str) -> u32 {
    return
        get_max_for_colour(line, "red") *
            get_max_for_colour(line, "green") *
            get_max_for_colour(line, "blue");
}

fn line_is_within_bounds(line: &str, red_bound: u32, green_bound: u32, blue_bound: u32) -> bool {
    return
        get_max_for_colour(line, "red") <= red_bound &&
            get_max_for_colour(line, "green") <= green_bound &&
            get_max_for_colour(line, "blue") <= blue_bound;
}

fn get_max_for_colour(line: &str, colour: &str) -> u32 {
    let pattern = r"(\d+) ".to_string() + colour;
    let re = Regex::new(&pattern).unwrap();

    let mut greatest = 0;
    for captures in re.captures_iter(line) {
        if let Some(num_match) = captures.get(1) {
            let num: u32 = FromStr::from_str(num_match.as_str()).unwrap();
            if greatest < num {
                greatest = num;
            }
        }
    }
    return greatest;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_can_calculate_sum_of_possible_games() {
        let content = std::fs::read_to_string("inputs/input2.txt")
            .expect("Unable to open file");
        assert_eq!(2439, part1(&content));
    }

    #[test]
    fn part2_can_calculate_power_of_games() {
        let content = std::fs::read_to_string("inputs/input2.txt")
            .expect("Unable to open file");
        assert_eq!(63711, part2(&content));
    }

    #[test]
    fn part1_can_calculate_sum_of_possible_games_for_tests() {
        let content = std::fs::read_to_string("inputs/input2_test.txt")
            .expect("Unable to open file");
        assert_eq!(8, part1(&content));
    }

    #[test]
    fn part2_can_calculate_power_of_games_for_tests() {
        let content = std::fs::read_to_string("inputs/input2_test.txt")
            .expect("Unable to open file");
        assert_eq!(2286, part2(&content));
    }

    #[test]
    fn can_calculate_power_of_game() {
        let line = "Game 71: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        assert_eq!(630, calculate_power_of_game(line));
    }

    #[test]
    fn can_get_game_number() {
        let line = "Game 71: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        assert_eq!(71, get_game_number(line));
    }

    #[test]
    fn can_determine_if_line_is_within_limits() {
        let line = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        assert_eq!(true, line_is_within_bounds(line, 14, 5, 16));
        assert_eq!(false, line_is_within_bounds(line, 14, 1, 15));
    }

    #[test]
    fn can_find_right_amount_per_colour() {
        let line = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        assert_eq!(14, get_max_for_colour(line, "red"));
        assert_eq!(3, get_max_for_colour(line, "green"));
        assert_eq!(15, get_max_for_colour(line, "blue"));
        assert_eq!(0, get_max_for_colour(line, "orange"));
    }
}
