advent_of_code::solution!(5);
use advent_of_code::*;
use std::collections::HashMap;
use std::collections::HashSet;

fn page_problems(page: &Vec<u32>, order_map: &HashMap<u32, HashSet<u32>>) -> Option<(u32, u32)> {
    let mut seen: Vec<(usize, u32)> = vec![];

    for (i, page_num) in page.into_iter().enumerate() {
        for (seen_pos, seen_before) in seen.iter() {
            let emptyset = HashSet::new();
            let prohibited = order_map.get(&seen_before).unwrap_or(&emptyset);
            if prohibited.contains(&page_num) {
                return Some((i as u32, *seen_pos as u32));
            }
        }
        seen.push((i, *page_num));
    }
    None
}

fn solve(rules: &Vec<SplitResult>, pages: &Vec<SplitResult>, fixer: Option<fn(broken: &Vec<u32>, order_map: &HashMap<u32, HashSet<u32>>) -> Option<Vec<u32>>>) -> Option<u32> {
    let mut order_map: HashMap<u32, HashSet<u32>> = HashMap::new();

    for rule_line in rules {
        match rule_line {
            SplitResult::Value(line) => {
                let Some((x, y)) = line.split_once("|") else { return None };
                order_map.entry(y.parse::<u32>().unwrap()).or_insert(HashSet::new()).insert(x.parse::<u32>().unwrap());
            },
            _ => panic!()
        }
    }

    Some(pages.iter().filter_map(|split_value| {
        let SplitResult::Value(page_line) = split_value else { return None };
        let page_split: Vec<u32> = page_line.split(",").map(|p| p.parse().unwrap()).collect();

        if let Some(fixer_fn) = fixer {
            let Some(fixed) = fixer_fn(&page_split, &order_map) else { return None };
            Some(fixed[fixed.len() / 2])
        } else if page_problems(&page_split, &order_map) == None {
            Some(page_split[page_split.len() / 2])
        } else {
            None
        }
    }).sum())
}

pub fn part_one(input: &str) -> Option<u32> {
    if let SplitResult::Result(items) = supersplit(input, &mut vec!["\n", "\n\n"]) {
        let [SplitResult::Result(rules), SplitResult::Result(pages)] = items.as_slice() else { return None };
        solve(rules, pages, None)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    if let SplitResult::Result(items) = supersplit(input, &mut vec!["\n", "\n\n"]) {
        let [SplitResult::Result(rules), SplitResult::Result(pages)] = items.as_slice() else { return None };
        solve(rules, pages, Some(|page: &Vec<u32>, order: &HashMap<u32, HashSet<u32>>| {
            let Some((i, j)) = page_problems(&page, &order) else { return None };
            let mut fixed = page.clone();
            fixed.swap(i as usize, j as usize);

            loop {
                let Some((x, y)) = page_problems(&fixed, &order) else { return Some(fixed) };
                fixed.swap(x as usize, y as usize);
            }
        }))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
