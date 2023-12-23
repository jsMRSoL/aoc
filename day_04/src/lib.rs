#![allow(unused)]
mod chrislib;
mod simon;
use std::collections::{BTreeMap, HashMap, HashSet};

use nom::bytes::complete::tag;
use nom::character::complete::{self, digit1, line_ending, space0, space1};
use nom::multi::{fold_many1, separated_list1};
use nom::sequence::{delimited, separated_pair, terminated, tuple};
use nom::IResult;
use nom::Parser;

#[derive(Debug)]
pub struct AocError;

#[derive(Debug)]
struct Card {
    winners: HashSet<u32>,
    mine: HashSet<u32>,
}

impl Card {
    fn score(&self) -> u32 {
        let power = self.winners.intersection(&self.mine).count() as u32;

        match power.checked_sub(1) {
            Some(num) => 2u32.pow(num),
            None => 0,
        }
    }

    fn match_count(&self) -> u32 {
        self.winners.intersection(&self.mine).count() as u32
    }
}

fn set(s: &str) -> IResult<&str, HashSet<u32>> {
    fold_many1(
        terminated(complete::u32, space0),
        HashSet::new,
        |mut acc: HashSet<u32>, num| {
            acc.insert(num);
            acc
        },
    )(s)
}

fn card(s: &str) -> IResult<&str, Card> {
    let (s, _) = delimited(
        tuple((tag("Card"), space1)),
        digit1,
        tuple((tag(":"), space1)),
    )(s)?;
    separated_pair(set, tuple((tag("|"), space1)), set)
        .map(|(winners, mine)| Card { winners, mine })
        .parse(s)
}

fn cards(s: &str) -> IResult<&str, Vec<Card>> {
    separated_list1(line_ending, card)(s)
}

fn process(s: &str) -> u32 {
    let result = match cards(s) {
        Ok((_, cds)) => cds.iter().map(|card| card.score()).sum::<u32>(),
        Err(_) => panic!("That wasn't supposed to happen..."),
    };
    result
}

fn collect_cards(s: &str) -> u32 {
    let cds = match cards(s) {
        Ok((_, cds)) => cds,
        Err(_) => panic!("That wasn't supposed to happen..."),
    };

    let mut card_no = 0u32;
    type MatchCount = u32;
    type Copies = u32;
    let mut collection: BTreeMap<u32, (MatchCount, Copies)> = BTreeMap::new();
    for cd in &cds {
        card_no += 1;
        let mc = cd.match_count();
        let copies = 1u32;
        collection.insert(card_no, (mc, copies));
    }

    let mut card_no = 0u32;
    let mut count = 0u32;
    for cd in cds {
        card_no += 1;
        let Some((mc, cc)) = collection.get(&card_no) else {
            continue;
        };
        let m_c = *mc;
        let c_c = *cc;
        for no in (card_no + 1)..(card_no + mc + 1) {
            let inc = c_c;
            collection.entry(no).and_modify(|(mc, cc)| *cc += c_c);
        }
        count += c_c;
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    fn lines(s: &str) -> IResult<&str, Vec<&str>> {
        let (s, cards) = separated_list1(line_ending, my_line)(s)?;
        Ok((s, cards))
    }

    fn my_line(s: &str) -> IResult<&str, &str> {
        nom::bytes::complete::take_till(|c| c == '\n')(s)
    }

    // why is this only producing one card?
    #[test]
    fn split_to_cards() {
        println!("output {res:?}", res = cards(TEST_INPUT));
    }

    #[test]
    fn split_to_lines() {
        println!("output {res:?}", res = lines(TEST_INPUT));
    }

    #[test]
    fn test_test_input() {
        let input = include_str!("./input.txt");
        assert_eq!(13, process(input));
    }

    #[test]
    fn test_process() {
        let input = TEST_INPUT;
        let _ = process(input);
        // assert_eq!(13, process(input));
    }

    #[test]
    fn test_line2_parse2() {
        let input = "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        println!("output {res:?}", res = card(input));
    }

    #[test]
    fn test() {}
    use rstest::rstest;

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    fn line_test(#[case] line: &str, #[case] expected: u32) {
        let (input, card) = card(line).expect("should be a valid card");
        assert_eq!(input, "");
        assert_eq!(expected, card.score())
    }

    #[test]
    fn part_01() {
        let input = include_str!("./input.txt");
        println!("{}", process(input));
    }

    #[test]
    fn test_process_chris() {
        let input = r"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(13, process(input));
    }

    #[test]
    fn test_collect_cards() {
        let input = TEST_INPUT;
        assert_eq!(30, collect_cards(input));
    }

    #[test]
    fn part_02() {
        let input = include_str!("./input.txt");
        println!("{}", collect_cards(input));
    }
}
