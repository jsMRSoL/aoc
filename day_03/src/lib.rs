#![allow(dead_code)]
use itertools::Itertools;
use std::collections::BTreeMap;
use std::ops::Add;

const TEST_INPUT: &str = r"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

// track location of all *'s
// track location of all digits
// digits should know their home address
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Self;

    fn add(self, other: (i32, i32)) -> Self {
        Self {
            x: self.x + other.0,
            y: self.y + other.1,
        }
    }
}

type Gears = Vec<Point>;
type DigitLocations = BTreeMap<Point, Point>;
type NumberLocations = BTreeMap<Point, i32>;

fn parse_input(s: &str) -> (Gears, DigitLocations, NumberLocations) {
    let mut gears: Gears = Vec::new();
    let mut digit_locs: DigitLocations = BTreeMap::new();
    let mut num_locs: NumberLocations = BTreeMap::new();
    // Holds an in progress PartNumber whist its digits are being parsed
    let mut num: u32 = 0;
    let mut num_origin: Option<Point> = None;

    for (y, line) in s.lines().enumerate() {
        let max_x = line.len();
        for (x, chr) in line.chars().enumerate() {
            // We only know we've completed a part number when we next see a non-digit
            // character. Check for that here and emit the `PartNumber`.
            if !chr.is_digit(10) || x == max_x {
                if let Some(Point {
                    y: root_y,
                    x: root_x,
                }) = num_origin
                {
                    num_locs.insert(Point::new(root_x as i32, root_y as i32), num as i32);
                }
                num = 0;
                num_origin = None;
            }

            match chr {
                // represents a blank space
                '.' => {}
                // build num digit by digit
                c if c.is_digit(10) => {
                    num_origin = num_origin.or(Some(Point {
                        x: x as i32,
                        y: y as i32,
                    }));
                    num = num * 10 + chr.to_digit(10).expect("tested with is_digit");
                    // log digit location
                    if let Some(Point {
                        x: root_x,
                        y: root_y,
                    }) = num_origin
                    {
                        digit_locs
                            .insert(Point::new(x as i32, y as i32), Point::new(root_x, root_y));
                    }
                }
                // log the locations of gears
                '*' => {
                    gears.push(Point::new(x as i32, y as i32));
                }
                _ => {}
            }
        }
    }

    (gears, digit_locs, num_locs)
}

struct InvalidGear;

fn lookup_gear_nos(
    loc: &Point,
    digit_locs: &DigitLocations,
    num_locs: &NumberLocations,
) -> Result<i32, InvalidGear> {
    let pts_ring = OFFSETS
        .iter()
        .map(|pt| {
            let offset: Point = *loc + *pt;
            offset
        })
        .collect::<Vec<_>>();
    let digits = pts_ring
        .iter()
        .map(|pt| {
            if let Some(nl) = digit_locs.get(pt) {
                Some(nl)
            } else {
                None
            }
        })
        .filter(|opt| opt.is_some())
        .unique()
        .collect::<Vec<_>>();

    if digits.len() == 2 {
        let sum: i32 = digits
            .into_iter()
            .map(|opt_digit| {
                if let Some(p) = opt_digit {
                    num_locs.get(p)
                } else {
                    None
                }
            })
            .fold(1, |acc, opt| acc * opt.unwrap_or(&1));
        return Ok(sum);
    }
    Err(InvalidGear)
}

#[cfg_attr(rustfmt, rustfmt_skip)]
const OFFSETS: [(i32, i32); 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    (0, -1),           (0,  1),
    (1, -1), (1, 0),   (1, 1),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = TEST_INPUT;
        let (gears, digit_locs, num_locs) = parse_input(input);
        println!("Gear locations:\n{:#?}", gears);
        println!("Digit locations:\n{:#?}", digit_locs);
        println!("Num locations:\n{:#?}", num_locs);
    }

    #[test]
    fn test_lookup_gear_nos_print() {
        let input = TEST_INPUT;
        let (gears, digit_locs, num_locs) = parse_input(input);
        for gear in gears {
            let _ = lookup_gear_nos(&gear, &digit_locs, &num_locs);
        }
    }

    #[test]
    fn test_part_02_test_input() {
        let input = TEST_INPUT;
        let (gears, digit_locs, num_locs) = parse_input(input);
        let total = gears.iter().fold(0, |acc, pt| {
            acc + lookup_gear_nos(&pt, &digit_locs, &num_locs).unwrap_or(0)
        });
        assert_eq!(total, 467835);
    }

    #[test]
    fn part_02() {
        let input = include_str!("./input.txt");
        let (gears, digit_locs, num_locs) = parse_input(input);
        let total = gears.iter().fold(0, |acc, pt| {
            acc + lookup_gear_nos(&pt, &digit_locs, &num_locs).unwrap_or(0)
        });
        println!("total: {total}");
    }
}
