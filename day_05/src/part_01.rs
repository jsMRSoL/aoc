use crate::{INPUT, TEST_INPUT};
use nom::branch::alt;
use nom::character::complete::u64;
use nom::combinator::eof;
use nom::{
    bytes::complete::{self, tag, take_until},
    character::complete::{line_ending, multispace0, newline, space0, space1},
    multi::many1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};
use std::ops::Range;

type Seeds = Vec<u64>;
type Maps = Vec<(Range<u64>, Range<u64>)>;
// type Maps = Vec<(u64, u64, u64)>;

fn parse_input(input: &str) -> IResult<&str, (Seeds, Vec<Maps>)> {
    let (input, seeds) = seeds(input)?;
    let (input, maps) = many1(map)(input)?;
    Ok((input, (seeds, maps)))
}

fn seeds(input: &str) -> IResult<&str, Seeds> {
    let (input, res) = tag("seeds:")(input)?;
    many1(preceded(space1, u64))(input)
}

fn map(input: &str) -> IResult<&str, Maps> {
    let (input, _heading) = tuple((take_until("map:"), tag("map:"), newline))(input)?;
    many1(terminated(line, alt((line_ending, eof))))(input)
}

fn line(input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (input, (dest, src, len)) =
        tuple((terminated(u64, space0), (terminated(u64, space0)), u64))(input)?;
    // a .. range is non-inclusive (so no -1 necessary)
    Ok((input, (src..(src + len), dest..(dest + len))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeds_works() {
        let input = "seeds: 79 14 55 13";
        let (tail, seeds) = match seeds(input) {
            Ok((t, s)) => (t, s),
            _ => panic!("Didn't get seeds out!"),
        };
        assert_eq!(vec![79, 14, 55, 13], seeds);
        println!("seeds {seeds:?}");
    }

    #[test]
    fn parse_input_works() {
        let input = TEST_INPUT;
        let (tail, (seeds, maps)) = match parse_input(input) {
            Ok(res) => res,
            Err(e) => panic!("{e}"),
        };
        assert_eq!("", tail);
        assert_eq!(vec![79, 14, 55, 13], seeds);
        assert_eq!(7, maps.len());
        let expected = vec![
            vec![(98..100, 50..52), (50..98, 52..100)],
            vec![(15..52, 0..37), (52..54, 37..39), (0..15, 39..54)],
            vec![
                (53..61, 49..57),
                (11..53, 0..42),
                (0..7, 42..49),
                (7..11, 57..61),
            ],
            vec![(18..25, 88..95), (25..95, 18..88)],
            vec![(77..100, 45..68), (45..64, 81..100), (64..77, 68..81)],
            vec![(69..70, 0..1), (0..69, 1..70)],
            vec![(56..93, 60..97), (93..97, 56..60)],
        ];
        assert_eq!(expected, maps);
    }

    #[test]
    fn map_works() {
        let input = "seed-to-soil map:
50 98 2
52 50 48";
        let result = map(input);
        println!("result {result:?}");
    }

    #[test]
    fn line_works() {
        let input = "50 98 2\n";
        let (range1, range2) = match line(input) {
            Ok((_, (r1, r2))) => (r1, r2),
            Err(e) => {
                eprintln!("Bad parsing!: {e}");
                panic!();
            }
        };
        assert_eq!((50..52, 98..100), (range2.clone(), range1.clone()));
        println!("Ranges\n{range1:?}, {range2:?}");
    }
}
