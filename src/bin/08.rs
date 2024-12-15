advent_of_code::solution!(8);
use advent_of_code::*;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_input(input);
    let antennae: Vec<Point> = grid.points().into_iter().filter_map(|(point, val)| if val != "." { Some(point) } else { None }).collect();

    Some(grid.points().into_iter().filter(|(origin, _val)| {
        antennae.iter().any(|antenna_point| {
            if antenna_point == origin { return false }
            let ray_dir = (antenna_point.clone() - origin.clone()) * 2;
            let antenna_val = grid.at(&antenna_point);
            let mut found = false;
            grid.raycast(origin, &ray_dir, |_p, v| {
                if v == antenna_val {
                    found = true;
                }
                true
            });
            found
        })
    }).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from_input(input);
    let antennae: Vec<Point> = grid.points().into_iter().filter_map(|(point, val)| if val != "." { Some(point) } else { None }).collect();
    let mut antinodes: HashSet<Point> = HashSet::new();
    for antenna_point in antennae.iter() {
        for other_antenna in antennae.iter() {
            if antenna_point == other_antenna { continue }
            let ray_dir =  other_antenna.clone() - antenna_point.clone();
            let antenna_val = grid.at(&antenna_point);
            let mut found = false;
            let mut first = true;
            grid.raycast(&antenna_point, &ray_dir, |p, v| {
                if v == antenna_val && first {
                    found = true;
                    first = false;
                }
                if found {
                    antinodes.insert(p.clone());
                    false
                } else {
                    true
                }
            });
        }
    }
    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
