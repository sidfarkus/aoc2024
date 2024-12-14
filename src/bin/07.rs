advent_of_code::solution!(7);
use advent_of_code::*;
use itertools::Itertools;

#[derive(Clone,Debug)]
enum Operation {
    Plus,
    Multiply,
    Concat
}

fn solve(nums: &Vec<u64>, ops: &Vec<Operation>) -> u64 {
    if nums.len() < 2 {
        return *nums.first().unwrap_or(&0);
    }

    nums.into_iter().skip(1).enumerate().fold(nums[0], |acc, (i, num)| match ops[i] {
        Operation::Plus => acc + num,
        Operation::Multiply => acc * num,
        Operation::Concat => (acc.to_string() + &num.to_string()).parse().unwrap()
    })
}

fn attempt(goal: u64, nums: &Vec<u64>, allow_concat: bool) -> bool {
    let all_ops = nums.into_iter().skip(1).map(|_| if allow_concat {
        vec![Operation::Plus, Operation::Multiply, Operation::Concat].into_iter()
    } else {
        vec![Operation::Plus, Operation::Multiply].into_iter()
    });
    for trial in all_ops.multi_cartesian_product() {
        let result = solve(nums, &trial);
        if result == goal {
            return true
        }
    }
    return false
}

pub fn part_one(input: &str) -> Option<u64> {
    let SplitResult::Result(lines) = supersplit(input, &mut vec![" ", ": ", "\n"]) else { return None };
    Some(lines.into_iter().fold(0, |acc, line| {
        let SplitResult::Result(parts) = line else {return 0};
        let [SplitResult::Result(ref sum_value), SplitResult::Result(ref num_values)] = parts[..] else { return 0 };
        let SplitResult::Value(ref sum) = sum_value[0] else { return 0 };
        let numbers: Vec<_> = num_values.into_iter().map(|n| match n {
            SplitResult::Value(x) => x.parse::<u64>().unwrap(),
            _ => 0
        }).collect();
        let goal: u64 = sum.parse().unwrap();
        acc + if attempt(goal, &numbers, false) {
            goal
        } else {
            0
        }
    }))
}

pub fn part_two(input: &str) -> Option<u64> {
    let SplitResult::Result(lines) = supersplit(input, &mut vec![" ", ": ", "\n"]) else { return None };
    Some(lines.into_iter().fold(0, |acc, line| {
        let SplitResult::Result(parts) = line else {return 0};
        let [SplitResult::Result(ref sum_value), SplitResult::Result(ref num_values)] = parts[..] else { return 0 };
        let SplitResult::Value(ref sum) = sum_value[0] else { return 0 };
        let numbers: Vec<_> = num_values.into_iter().map(|n| match n {
            SplitResult::Value(x) => x.parse::<u64>().unwrap(),
            _ => 0
        }).collect();
        let goal: u64 = sum.parse().unwrap();
        acc + if attempt(goal, &numbers, true) {
            goal
        } else {
            0
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
