const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

const RED_STRING: &str = "red";
const GREEN_STRING: &str = "green";
const BLUE_STRING: &str = "blue";

pub fn process(input: &str) -> String {
    let output = input
        .lines()
        .map(|line| {
            let two_split = line.split(':').collect::<Vec<_>>();
            let is_game_possible = nested_games_possible(two_split[1]);
            if !is_game_possible {
                return 0;
            };
            get_game_id(two_split[0])
        })
        .sum::<u32>();

    output.to_string()
}

fn nested_games_possible(line: &str) -> bool {
    let mut chunks = line.split(';');
    for nested in chunks.by_ref() {
        let ball = nested.split(',');
        for bal in ball.into_iter() {
            let number = bal
                .split_whitespace()
                .next()
                .unwrap()
                .parse::<u32>()
                .unwrap();
            let color = bal
                .split_whitespace()
                .nth(1)
                .unwrap()
                .parse::<String>()
                .unwrap();
            match (color.as_str(), number) {
                (BLUE_STRING, number) if number > BLUE => return false,
                (GREEN_STRING, number) if number > GREEN => return false,
                (RED_STRING, number) if number > RED => return false,
                _ => (),
            }
        }
    }
    true
}

fn get_game_id(line: &str) -> u32 {
    line.split_whitespace()
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("8", process(input));
    }
}
