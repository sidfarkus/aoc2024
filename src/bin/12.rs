advent_of_code::solution!(12);
use advent_of_code::*;
use std::collections::HashSet;
pub struct CornerAdjacency;
impl Adjacency for CornerAdjacency {
  fn relative_locations(&self) -> impl Iterator<Item = RelativeLocation> {
    vec![
      RelativeLocation { name: Some("upper left"), offsets: vec![
        Point {x: -1, y: -1},
        Point {x: 0, y: -1},
        Point {x: -1, y: 0},
      ]},
      RelativeLocation { name: Some("upper right"), offsets: vec![
        Point {x: 1, y: -1},
        Point {x: 0, y: -1},
        Point {x: 1, y: 0},
      ]},
      RelativeLocation { name: Some("lower right"), offsets: vec![
        Point {x: 1, y: 1},
        Point {x: 0, y: 1},
        Point {x: 1, y: 0},
      ]},
      RelativeLocation { name: Some("lower left"), offsets: vec![
        Point {x: -1, y: 1},
        Point {x: 0, y: 1},
        Point {x: -1, y: 0},
      ]},
    ].into_iter()
  }
}

fn regions(grid: &Grid) -> Vec<HashSet<Point>> {
    let mut visited: HashSet<Point> = HashSet::new();
    grid.points().into_iter().fold(vec![], |mut acc, (point, _)| {
        if visited.contains(&point) { return acc }

        let region = grid.flood(&point);
        for point in &region {
            visited.insert(point.clone());
        }

        acc.push(region);
        acc
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::from_input(input);
    let mut visited: HashSet<Point> = HashSet::new();
    Some(regions(&grid).into_iter().fold(0, |price, region| {
        let mut fences: u32 = 4 * region.len() as u32;
        for point in &region {
            let region_neighbors: Vec<Point> = grid.neighbors(point.clone(), FourWayAdjacency)
                .into_iter()
                .filter_map(|n| if n.has_value() && region.contains(n.point()) { Some(n.point().clone())} else { None })
                .collect();
            fences -= region_neighbors.len() as u32;
            for n in region_neighbors.into_iter() {
                visited.insert(n);
            }
        }
        price + fences * region.len() as u32
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::from_input(input);
    Some(regions(&grid).into_iter().fold(0, |price, region| {
        let corners = region.iter().fold(0, |acc, point| {
            let c = grid.neighbors(point.clone(), CornerAdjacency)
                .into_iter()
                .filter(|n| {
                    let center_val = grid.at(&point);
                    let is_outside_corner = n.values.iter().all(|v| v != center_val) ||
                        n.values.len() >= 2 && n.values.iter().skip(1).all(|v| v != center_val);
                    let is_inside_corner = n.values.len() == 3 && n.value() != center_val && &n.values[1] == center_val && &n.values[2] == center_val;
                    is_outside_corner || is_inside_corner
                })
                .count();
            acc + c
        });
        price + (corners * region.len()) as u32
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
