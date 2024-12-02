advent_of_code::solution!(1);
use itertools::Itertools;
use std::convert::TryFrom;

pub fn part_one(input: &str) -> Option<i64> {
    let (mut first, mut second): (Vec<i64>, Vec<i64>) = input.lines().map(|line| line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect_tuple().unwrap()).unzip();
    first.sort();
    second.sort();
    Some(first.iter().zip(second.iter()).map(|(f, s)| (f - s).abs()).sum())
}

pub fn part_two(input: &str) -> Option<i64> {
    let (mut first, mut second): (Vec<i64>, Vec<i64>) = input.lines().map(|line| line.split_whitespace().map(|x| x.parse::<i64>().unwrap()).collect_tuple().unwrap()).unzip();
    let lookup = second.iter().counts();
    lookup.get(&0).unwrap_or(&1);
    Some(first.iter().map(|&x| (*lookup.get(&x).unwrap_or(&0)) as i64 * x).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
