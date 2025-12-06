use day_01::Turn;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::tag;

type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn parse_line(input: &str) -> IResult<&str, Turn> {
    let (input, (direction, count)) =
        (alt((tag("L"), tag("R"))), nom::character::complete::i32).parse(input)?;
    match direction {
        "L" => Ok((input, Turn::Left(count))),
        "R" => Ok((input, Turn::Right(count))),
        _ => unreachable!(),
    }
}

fn get_turns(input: &str) -> MyResult<Vec<Turn>> {
    let instructions = input
        .lines()
        .filter_map(|line| parse_line(line).ok().map(|(_, turn)| turn))
        .collect::<Vec<Turn>>();

    Ok(instructions)
}

fn count_all_zeroes(input: &str) -> MyResult<i32> {
    let mut dial = 50;
    let mut zero_count = 0;
    let turns = get_turns(input)?;

    for turn in turns {
        match turn {
            Turn::Left(n) => {
                for _ in 0..n {
                    dial -= 1;
                    if dial == 0 {
                        zero_count += 1;
                    }
                    if dial < 0 {
                        dial += 100;
                    }
                }
            }
            Turn::Right(n) => {
                for _ in 0..n {
                    dial += 1;
                    if dial > 99 {
                        dial -= 100;
                    }
                    if dial == 0 {
                        zero_count += 1;
                    }
                }
            }
        }
        dbg!("End of turn:", dial, zero_count);
    }
    Ok(zero_count)
}

fn main() {
    let input = include_str!("../input1.txt");
    match count_all_zeroes(input) {
        Ok(c) => println!("Count is: {c}"),
        Err(e) => eprintln!("{e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input() {
        let input = include_str!("../input1.txt");
        let res = get_turns(input);
        assert!(res.is_ok());
    }

    #[test]
    fn test_example() {
        let input = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let res = count_all_zeroes(input);
        let _ = dbg!(res);
    }
}
