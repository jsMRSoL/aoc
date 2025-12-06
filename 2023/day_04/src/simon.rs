
use std::collections::HashSet;

use nom::bytes::complete::{tag, take_until};
use nom::character::complete::space0;
use nom::multi::many0;
use nom::sequence::{delimited, terminated, tuple};
use nom::IResult;
use nom::{
    character::complete,
    combinator::map,
};


const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

type Winners = Vec<u32>;
type Picks = Vec<u32>;

fn parse_input(s: &str) -> IResult<&str, Vec<(Winners, Picks)>> {
    let prefix_fn = delimited(take_until(":"), tag(":"), space0);
    let nums_fn = many0(terminated(complete::u32, space0));
    let sep_fn = delimited(space0, tag("|"), space0);
    let mine_fn = many0(terminated(complete::u32, space0));
    let line_fn = tuple((prefix_fn, nums_fn, sep_fn, mine_fn));
    let drop_unwanted = map(line_fn, |(_, winners, _, picks)| (winners, picks));
    let (input, output) = many0(drop_unwanted)(s)?;

    Ok((input, output))
}

fn process(s: &str) -> u32 {
    let mut total = 0u32;
    let (_, data) = parse_input(s).expect("parsing should just work!");
    for (winners, picks) in data {
        let winners_set: HashSet<&u32> = HashSet::from_iter(winners.iter());
        let picks_set: HashSet<&u32> = HashSet::from_iter(picks.iter());
        let mut power = winners_set.intersection(&picks_set).count() as u32;
        let score = match power {
            0 => 0,
            _ => 2u32.pow(power - 1),
        };
        total += score;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = TEST_INPUT;
        println!("output {output:?}", output = parse_input(input));
    }

    #[test]
    fn test_parse_input_test_input() {
        let input = TEST_INPUT;
        assert_eq!(13, process(input));
    }

    #[test]
    fn part_01() {
        let input = include_str!("./input.txt");
        println!("total {output:?}", output = process(input));
    }
}
