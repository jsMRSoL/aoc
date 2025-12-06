#![allow(dead_code)]

use std::option::Option;

#[derive(Debug, PartialEq)]
struct NumStrNotFound;

fn sum_str_digits(input: &str) -> i32 {
    let mut first = 'a';
    let mut last = 'z';
    let mut sum = 0;
    for line in input.lines() {
        if !line.is_empty() {
            for c in line.chars() {
                if c.is_numeric() {
                    first = c;
                    break;
                }
            }
            for c in line.chars().rev() {
                if c.is_numeric() {
                    last = c;
                    break;
                }
            }
            let figure = format!("{first}{last}").parse::<i32>().unwrap_or(0);
            sum += figure;
        }
    }
    sum
}

fn sum_str_digits_fnl(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            let first = l
                .chars()
                .find(|c| c.is_numeric())
                .expect("first numeric char must exist");
            let last = l
                .chars()
                .rev()
                .find(|c| c.is_numeric())
                .expect("last numeric char must exist");
            format!("{first}{last}").parse().unwrap_or(0)
        })
        .sum()
}

fn sum_alpha_nums(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            let mut nums = l
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if c.is_numeric() {
                        Some(c)
                    } else {
                        match_num_str(&l[i..]).ok()
                    }
                })
                .filter(Option::is_some);

            let first = nums
                .next()
                .expect("nums iterator to produce options")
                .expect("first must exist");
            let try_last = nums.last();
            let last: char;
            if let Some(Some(c)) = try_last {
                last = c;
            } else {
                last = first;
            }

            format!("{first}{last}").parse().unwrap_or(0)
        })
        .sum()
}
fn match_num_str(s: &str) -> Result<char, NumStrNotFound> {
    if s.starts_with("one") {
        return Ok('1');
    }
    if s.starts_with("two") {
        return Ok('2');
    }
    if s.starts_with("three") {
        return Ok('3');
    }
    if s.starts_with("four") {
        return Ok('4');
    }
    if s.starts_with("five") {
        return Ok('5');
    }
    if s.starts_with("six") {
        return Ok('6');
    }
    if s.starts_with("seven") {
        return Ok('7');
    }
    if s.starts_with("eight") {
        return Ok('8');
    }
    if s.starts_with("nine") {
        return Ok('9');
    }
    Err(NumStrNotFound)
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::rstest;

    const INPUT: &str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const INPUT2: &str = r"gsjgklneight6zqfz
7one718onegfqtdbtxfcmd
xvtfhkm8c9
914two8
vxzzvdhfqfsix83c1ttvbbstxgdrkfcnmm3
76mkvhmbkpm
8sixssmlzlhrnineggmrvg6
threeninedtr7219
two2geight
3nine9fivetwo9twohxhc8
llbfmnzzntdcfbslcl3xxvz
two3leighttvpkfmjhhonefour
1b9four
fivefdlqonesj2six
hfptgztwosix8
fourkcxqfgxbsvjj3472
xhzs7rdphtxhtwo
eightthree51
3nmronemlqzfxgonepkh
9vnxqtjjrsg";
    // 533fivesixrvqfxjrdhl
    // bkrljtkjb42fqnp
    // 4mrh734";
    //
    const PART2_TEST_INPUT: &str = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
    // Part 1 tests
    #[rstest]
    #[case(INPUT, 142i32)]
    #[case(INPUT2, 1202i32)]
    fn part_01_cases(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(expected, sum_str_digits(input));
    }

    #[test]
    fn part_01() {
        let input = include_str!("./input.txt");
        let total = sum_str_digits(input);
        assert_eq!(55108, total);
    }

    #[test]
    fn part_01_fnl() {
        let input = include_str!("./input.txt");
        let res = sum_str_digits_fnl(input);
        assert_eq!(55108, res);
    }

    // Part 2 tests
    #[test]
    fn test_match_first_num_str() {
        let s = "eightwothree";
        assert_eq!(Ok('8'), match_num_str(s));
        assert_eq!(Err(NumStrNotFound), match_num_str("1234"));
    }

    #[test]
    fn test_match_all_num_strs() {
        let s = "eightwothree";
        let mut found: Vec<char> = vec![];
        for i in 0..(s.len() - 1) {
            match match_num_str(&s[i..]) {
                Ok(n) => found.push(n),
                Err(_) => (),
            }
        }
        assert_eq!(vec!['8', '2', '3'], found);
    }

    #[test]
    fn part_02() {
        let result = include_str!("./input.txt");
        let res = sum_alpha_nums(result);
        assert_eq!(56324, res);
    }

    #[test]
    fn part_02_dry_run() {
        let input = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let res = sum_alpha_nums(input);
        assert_eq!(281, res);
    }
}
