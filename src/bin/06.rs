advent_of_code::solution!(6);
use advent_of_code::*;
use itertools::Itertools;
use std::collections::HashSet;

fn visited(grid: &Grid) -> (Vec<Ray>, bool) {
    let mut is_cycle = false;
    let mut loop_detector: HashSet<Ray> = HashSet::new();
    let mut visited: Vec<Ray> = Vec::with_capacity((grid.height * grid.width) as usize);
    let Some((mut start, _)) = grid.points().into_iter().find(|(_, v)| v == "^") else { return (visited, false)};
    let mut dir = Point { x: 0, y: -1 };
    loop {
        match grid.raycast(&start, &dir.clone(), |walk: &Point, val: &String| {
            if val != "#" {
                let ray = Ray { origin: walk.clone(), dir: dir.clone() };
                if loop_detector.contains(&ray) {
                    is_cycle = true;
                    return true;
                }
                loop_detector.insert(ray.clone());
                visited.push(ray);
            } else {
                dir = Point { x: -dir.y, y: dir.x };
            }
            val == "#"
        }) {
            Some(next_point) => if is_cycle { break } else { start = next_point },
            _ => break
        }
    }
    (visited, is_cycle)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_input(input);
    let unique_visited = visited(&grid).0.into_iter().unique_by(|Ray {origin, dir: _}| origin.clone());
    Some(unique_visited.count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from_input(input);
    let candidates = visited(&grid).0.into_iter().unique_by(|Ray {origin, dir: _}| origin.clone());
    Some(candidates.fold(0, |acc, unique_location| {
        let mut new_grid = grid.clone();
        new_grid.set(&unique_location.origin, &String::from("#"));
        let (_trial_path, is_cycle) = visited(&new_grid);
        if is_cycle {
            acc + 1
        } else {
            acc
        }
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
