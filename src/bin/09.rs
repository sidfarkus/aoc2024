advent_of_code::solution!(9);
use advent_of_code::*;
use itertools::Itertools;
use itertools::MinMaxResult::{OneElement, MinMax};

fn init_mem(input: &str) -> Vec<i32> {
    let mut memory = Vec::with_capacity(100000);
    for (i, size) in input.chars().map(|c| c.to_string().parse::<i32>().unwrap()).enumerate() {
        let val: i32 = if i % 2 == 0 {
            (i as i32) / 2
        } else {
            -1
        };
        for _ in std::iter::repeat_n(val, size as usize) {
            memory.push(val);
        }
    }
    memory
}

fn checksum(mem: Vec<i32>) -> i64 {
    mem.into_iter().enumerate().filter(|(_i, n)| n >= &0).fold(0 as i64, |acc, (i, num)| {
        acc + (num * i as i32) as i64
    })
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut memory: Vec<i32> = init_mem(input);
    let mut head: usize = 0;
    let mut tail: usize = memory.len() - 1;
    loop {
        while memory[head] >= 0 && head < memory.len() { head += 1 }
        while memory[tail] < 0 && tail > 0 { tail -= 1 }
        if head >= tail { break }
        memory.swap(head, tail);

    }
    Some(checksum(memory))
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut memory: Vec<i32> = init_mem(input);
    let mut free: Vec<std::ops::RangeInclusive<usize>> = memory.iter().enumerate().chunk_by(|(_i, n)| **n).into_iter().filter_map(|(key, chunk)| {
        if key == -1 {
            match chunk.map(|(i, _)| i).minmax() {
                MinMax(min, max) => Some(min..=max),
                OneElement(elem) => Some(elem..=elem),
                _ => None
            }
        } else { None }
    }).collect();
    let mut tail = memory.len() - 1 .. memory.len();
    loop {
        if free.is_empty() { break; }
        while memory[tail.start] == -1 && tail.start - 1 > 0 { tail.start -= 1; tail.end = tail.start + 1 }
        while memory[tail.start - 1] == memory[tail.end - 1] && tail.start - 1 > 0 { tail.start -= 1 }
        if tail.start == 1 { break; }
        let Some(next_free) = free.iter().position(|range| range.size() >= tail.size() && *range.start() < tail.start) else {
            tail.start -= 1;
            tail.end = tail.start + 1;
            continue
        };
        let free_range = &free[next_free];
        for i in tail.clone() {
            memory.swap(i, *free_range.start() + (i - tail.start))
        }
        let leftover_space = free_range.size() - tail.size();
        if leftover_space > 0 {
            free[next_free] = *free_range.start() + tail.size()..=*free_range.end();
        } else {
            free.remove(next_free);
        }
        tail.start -= 1;
        tail.end = tail.start + 1;
    }
    Some(checksum(memory))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
