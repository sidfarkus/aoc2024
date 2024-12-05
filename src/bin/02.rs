advent_of_code::solution!(2);
use itertools::Itertools;

fn is_safe(report: &Vec<i32>) -> bool {
    let diffs: Vec<i32> = report.iter().tuple_windows().map(|(a, b)| a - b).collect();
    let lessthan = diffs.iter().all(|diff| diff.abs() <= 3);
    let signs = diffs.iter().map(|&x| x.signum()).all_equal();
    lessthan && signs
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = input.lines().map(|line| line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>());
    let safe = reports.filter(is_safe);
    Some(safe.count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = input.lines().map(|line| line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>());
    let safe = reports.filter(|report| {
        report.into_iter().combinations(report.len() - 1).any(|trial: Vec<&i32>| is_safe(&trial.into_iter().map(|&x| x).collect::<Vec<i32>>()))
    });
    Some(safe.count() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
