use std::cmp;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Game {
    id: i32,
    reds: i32,
    blues: i32,
    greens: i32,
}

#[derive(Debug, PartialEq)]
struct BadGameStr(String);

impl FromStr for Game {
    type Err = BadGameStr;
    fn from_str(s: &str) -> Result<Game, BadGameStr> {
        let mut head_tail = s.split(": ");
        let Some(head) = head_tail.next() else {
            return Err(BadGameStr(s.to_owned()));
        };
        let head = head.split(" ");
        let Some(id) = head.last() else {
            return Err(BadGameStr(s.to_owned()));
        };
        let id = id.parse().map_err(|_| BadGameStr(s.to_owned()))?;

        let mut reds = 0;
        let mut blues = 0;
        let mut greens = 0;

        let Some(tail) = head_tail.last() else {
            return Err(BadGameStr(s.to_owned()));
        };
        let hands = tail.split("; ");
        for hand in hands {
            let mut colours = hand.split(", ");
            while let Some(col_str) = colours.next() {
                let mut score_col = col_str.split(" ");
                let mut score = 0;
                if let Some(score_str) = score_col.next() {
                    let res = score_str.parse::<i32>();
                    score = res.unwrap_or(0);
                };
                match score_col.last() {
                    Some(c) if c == "red" => reds = cmp::max(reds, score),
                    Some(c) if c == "blue" => blues = cmp::max(blues, score),
                    Some(c) if c == "green" => greens = cmp::max(greens, score),
                    _ => {}
                }
            }
        }

        Ok(Game {
            id,
            reds,
            blues,
            greens,
        })
    }
}

fn count_valid_ids(s: &str) -> i32 {
    s.lines()
        .map(|l| {
            let r = l.parse::<Game>();
            match r {
                Ok(g) if g.reds < 13 && g.greens < 14 && g.blues < 15 => g.id,
                _ => 0,
            }
        })
        .sum()
}

fn powers(s: &str) -> i32 {
    s.lines()
        .map(|l| {
            let r = l.parse::<Game>();
            match r {
                Ok(g) => g.reds * g.greens * g.blues,
                _ => 0,
            }
        })
        .sum()
}

const TEST_INPUT: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_a_game_works() {
        let test_str = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        println!("{:?}", test_str.parse::<Game>());

        let res = test_str.parse::<Game>().unwrap();
        assert_eq!(
            Game {
                id: 2,
                reds: 1,
                blues: 4,
                greens: 3
            },
            res
        );
    }

    #[test]
    fn parsing_input_works() {
        for line in TEST_INPUT.lines() {
            println!("{:?}", line.parse::<Game>());
        }
    }

    #[test]
    fn test_count_valid_ids_test_input() {
        assert_eq!(8, count_valid_ids(TEST_INPUT));
    }

    #[test]
    fn part_01() {
        let input = include_str!("./input.txt");
        println!("{}", count_valid_ids(input));
    }

    #[test]
    fn test_powers_work_test_input() {
        assert_eq!(2286, powers(TEST_INPUT));
    }
    
    #[test]
    fn part_02() {
        let input = include_str!("./input.txt");
        println!("{}", powers(input));
    }
}
