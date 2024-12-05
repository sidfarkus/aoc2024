advent_of_code::solution!(3);
use regex::Regex;

pub fn part_one(input: &str) -> Option<i32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let sum = re.captures_iter(input).map(|caps| caps.extract()).fold(0, |acc: i32, (_, [rhs, lhs]): (&str, [&str; 2])| {
        acc + rhs.parse::<i32>().unwrap() * lhs.parse::<i32>().unwrap()
    });
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    Some(input.split("don't()").enumerate().fold(0, |acc: i32, (i, substr)| {
        let enabled = match (i, substr.find("do()")) {
            (0, _) => substr,
            (_, Some(start)) => &substr[start + 4..],
            (_, None) => ""
        };
        acc + part_one(enabled).unwrap()
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
