use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::multispace0;
use nom::combinator::all_consuming;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;

type MyResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct IdRange {
    start: u64,
    end: u64,
}

fn parse_input(input: &str) -> IResult<&str, Vec<IdRange>> {
    separated_list1(
        tag(","),
        terminated(
            separated_pair(complete::u64, tag("-"), complete::u64)
                .map(|(start, end)| IdRange { start, end }),
            multispace0,
        ),
    )
    .parse(input)
}

fn process_input(input: &'static str) -> MyResult<()> {
    let (_, id_ranges) = all_consuming(parse_input).parse(input)?;
    println!("printing...");
    println!("{id_ranges:#?}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../input.txt");
        let res = process_input(input);
        match res {
            Ok(_) => println!("It worked!"),
            Err(e) => eprintln!("Error: {e}"),
        }
    }
}
