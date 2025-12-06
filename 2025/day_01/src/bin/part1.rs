use day_01::MyResult;
use day_01::Turn;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::tag;

fn parse_line(input: &str) -> IResult<&str, Turn> {
    let (input, (direction, count)) =
        (alt((tag("L"), tag("R"))), nom::character::complete::i32).parse(input)?;
    let count = count % 100;
    match direction {
        "L" => Ok((input, Turn::Left(count))),
        "R" => Ok((input, Turn::Right(count))),
        _ => unreachable!(),
    }
}

fn get_turns() -> MyResult<Vec<Turn>> {
    let input = include_str!("../input1.txt");
    let instructions = input
        .lines()
        .filter_map(|line| parse_line(line).ok().map(|(_, turn)| turn))
        .collect::<Vec<Turn>>();

    Ok(instructions)
}

pub fn count_zeroes() -> Result<i32, Box<dyn std::error::Error>> {
    let mut dial = 50;
    let mut zero_count = 0;
    let turns = get_turns()?;
    for turn in turns {
        match turn {
            Turn::Left(n) => {
                dial -= n;
                if dial < 0 {
                    dial += 100;
                }
            }
            Turn::Right(n) => {
                dial += n;
                if dial > 99 {
                    dial -= 100;
                }
            }
        }
        if dial == 0 {
            zero_count += 1;
        }
    }
    Ok(zero_count)
}

fn main() {
    match count_zeroes() {
        Ok(c) => println!("Count is: {c}"),
        Err(e) => eprintln!("{e}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer_part_1() {
        let res = count_zeroes();
        assert!(res.is_ok());
        match res {
            Ok(answer) => println!("Answer is {answer}"),
            Err(e) => eprintln!("Uh oh! {e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input() {
        let res = get_turns();
        assert!(res.is_ok());
    }
}
