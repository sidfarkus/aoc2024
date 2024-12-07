advent_of_code::solution!(4);
use advent_of_code::*;

struct XmasAdjacency;
impl Adjacency for XmasAdjacency {
  fn relative_locations(&self) -> impl Iterator<Item = RelativeLocation> {
    vec![
      RelativeLocation { name: Some("diagonal up left"), offsets: vec![
        Point {x: -1, y: -1},
        Point {x: -2, y: -2},
        Point {x: -3, y: -3},
      ]},
      RelativeLocation { name: Some("up"), offsets: vec![
        Point {x: 0, y: -1},
        Point {x: 0, y: -2},
        Point {x: 0, y: -3},
      ]},
      RelativeLocation { name: Some("diagonal up right"), offsets: vec![
        Point {x: 1, y: -1},
        Point {x: 2, y: -2},
        Point {x: 3, y: -3},
      ]},
      RelativeLocation { name: Some("right"), offsets: vec![
        Point {x: 1, y: 0},
        Point {x: 2, y: 0},
        Point {x: 3, y: 0},
      ]},
      RelativeLocation { name: Some("diagonal down right"), offsets: vec![
        Point {x: 1, y: 1},
        Point {x: 2, y: 2},
        Point {x: 3, y: 3},
      ]},
      RelativeLocation { name: Some("down"), offsets: vec![
        Point {x: 0, y: 1},
        Point {x: 0, y: 2},
        Point {x: 0, y: 3},
      ]},
      RelativeLocation { name: Some("diagonal down left"), offsets: vec![
        Point {x: -1, y: 1},
        Point {x: -2, y: 2},
        Point {x: -3, y: 3},
      ]},
      RelativeLocation { name: Some("left"), offsets: vec![
        Point {x: -1, y: 0},
        Point {x: -2, y: 0},
        Point {x: -3, y: 0},
      ]},
    ].into_iter()
  }
}

struct MasAdjacency;
impl Adjacency for MasAdjacency {
  fn relative_locations(&self) -> impl Iterator<Item = RelativeLocation> {
    vec![
      RelativeLocation { name: Some("diagonal left"), offsets: vec![
        Point {x: -1, y: -1},
        Point {x: 1, y: 1},
      ]},
      RelativeLocation { name: Some("diagonal right"), offsets: vec![
        Point {x: 1, y: -1},
        Point {x: -1, y: 1},
      ]},
    ].into_iter()
  }
}

pub fn part_one(input: &str) -> Option<i32> {
    let grid = Grid::from_input(input);
    Some(grid.points().into_iter().fold(0, |acc: i32, (point, val)| {
        if val == "X" {
            acc + grid.neighbors(point, XmasAdjacency).iter().filter(|neighbor| neighbor.values.join("") == "MAS").count() as i32
        } else {
            acc
        }
    }))
}

pub fn part_two(input: &str) -> Option<i32> {
    let grid = Grid::from_input(input);
    Some(grid.points().into_iter().fold(0, |acc: i32, (point, val)| {
        if val == "A" {
            acc + grid.neighbors(point, MasAdjacency).iter().all(|n| n.values.join("") == "MS" || n.values.join("") == "SM") as i32
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
