use common::{Answer, Solution};

pub struct Day01;

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

impl Solution for Day01 {
    fn name(&self) -> &'static str {
        "Trebuchet?!"
    }

    fn part_a(&self, input: &str) -> Answer {
        let mut sum = 0;
        for line in input.lines() {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            sum += first * 10 + last;
        }

        sum.into()
    }

    fn part_b(&self, input: &str) -> Answer {
        let mut sum = 0;
        for line in input.lines() {
            let digits = digits(line);
            sum += digits[0] * 10 + digits[1];
        }

        sum.into()
    }
}

fn digits(i: &str) -> [u32; 2] {
    let mut first = None;
    let mut last = 0;

    let mut digit = |c| {
        first = first.or(Some(c));
        last = c;
    };

    let chars = i.as_bytes();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if c.is_ascii_digit() {
            digit((c - b'0') as u32);
        } else {
            for (j, d) in DIGITS.iter().enumerate() {
                if chars[i..].starts_with(d.as_bytes()) {
                    digit(j as u32 + 1);
                }
            }
        }
        i += 1;
    }

    [first.unwrap(), last]
}

#[cfg(test)]
mod test {
    use common::Solution;
    use indoc::indoc;

    use super::Day01;

    const CASE_A: &str = indoc! {"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
    "};

    const CASE_B: &str = indoc! {"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
    "};

    #[test]
    fn part_a() {
        assert_eq!(Day01.part_a(CASE_A), 142.into());
    }

    #[test]
    fn part_b() {
        assert_eq!(Day01.part_b(CASE_B), 281.into());
    }
}
