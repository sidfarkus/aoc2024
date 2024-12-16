advent_of_code::solution!(10);
use advent_of_code::*;
use std::collections::HashSet;

fn head_score(grid: &Grid, visited: &mut Option<&mut HashSet<Point>>, location: Point) -> u32 {
    let this_value: u32 = grid.at(&location).parse().unwrap();
    if this_value == 9 {
        let Some(ref mut visited_tracker) = visited.as_mut() else { return 1 };
        if visited_tracker.contains(&location) {
            return 0
        }
        visited_tracker.insert(location.clone());
        return 1
    }

    grid.neighbors(location, FourWayAdjacency).into_iter().map(|n| {
        if !n.values.is_empty() && n.values[0].parse::<u32>().unwrap() == this_value + 1 {
            head_score(grid, visited, n.points[0].clone())
        } else {
            0
        }
    }).sum()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_input(input);
    Some(grid.points().iter().map(|(p, v)| {
        if v == "0" {
            let mut visited: HashSet<Point> = HashSet::new();
            head_score(&grid, &mut Some(&mut visited), p.clone())
        } else {
            0
        }
    }).sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from_input(input);
    Some(grid.points().iter().map(|(p, v)| {
        if v == "0" {
            head_score(&grid, &mut None, p.clone())
        } else {
            0
        }
    }).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
