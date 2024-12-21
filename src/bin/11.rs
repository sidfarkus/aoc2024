advent_of_code::solution!(11);
use advent_of_code::*;
use memoize::memoize;

#[memoize]
fn score(start: u128, depth: u32) -> u128 {
    if depth <= 0 {
        1
    } else if start == 0 {
        score(1, depth - 1)
    } else if (start.ilog10() + 1) % 2 == 0 {
        let item_str = start.to_string();
        let (first, second) = item_str.split_at(item_str.len() / 2);
        score(first.parse().unwrap(), depth - 1) + score(second.parse().unwrap(), depth - 1)
    } else {
        score(start * 2024, depth - 1)
    }
}

pub fn part_one(input: &str) -> Option<u128> {
    Some(input.split(" ").map(|x| score(x.parse::<u128>().unwrap(), 25)).sum())
}

pub fn part_two(input: &str) -> Option<u128> {
    Some(input.split(" ").map(|x| score(x.parse::<u128>().unwrap(), 75)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
