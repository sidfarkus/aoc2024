pub mod template;
use std::ops;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Point {
  pub x: i32,
  pub y: i32
}

impl ops::Sub for Point {
  type Output = Self;

  fn sub(self, rhs: Point) -> Self::Output  {
    Point { x: self.x - rhs.x, y: self.y - rhs.y }
  }
}

impl ops::Mul<i32> for Point {
  type Output = Self;

  // Required method
  fn mul(self, rhs: i32) -> Self::Output {
    Point { x: self.x * rhs, y: self.y * rhs}
  }
}

use Point as Vector;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Ray {
  pub origin: Point,
  pub dir: Vector
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Grid {
  pub width: i32,
  pub height: i32,
  data: Vec<Vec<String>>
}

#[derive(Clone)]
pub struct RelativeLocation {
  pub offsets: Vec<Point>,
  pub name: Option<&'static str>
}

pub trait Adjacency {
  fn relative_locations(&self) -> impl Iterator<Item = RelativeLocation>;
}

fn points_for_location(origin: &Point, location: &RelativeLocation) -> Vec<Point> {
  location.offsets.iter().map(|point| Point {x: origin.x + point.x, y: origin.y + point.y}).collect()
}

pub struct Neighbor {
  pub location: RelativeLocation,
  pub values: Vec<String>
}

impl Grid {
  pub fn make(width: i32, height: i32, initial: Option<String>) -> Grid {
    Grid {
      width,
      height,
      data: (0..height).map(|_| {
        (0..width).map(|_| (&initial).clone().unwrap_or(String::from(""))).collect()
      }).collect::<Vec<Vec<String>>>()
    }
  }

  pub fn from_input(input: &str) -> Grid {
    let data: Vec<Vec<String>> = input.lines().map(|line| {
      let mut row = line.split("").skip(1).map(String::from).collect::<Vec<_>>();
      row.pop();
      row
    }).collect();
    Grid {
      width: data[0].len() as i32,
      height: data.len() as i32,
      data
    }
  }

  pub fn set(&mut self, p: &Point, val: &String)-> &Grid {
    self.data[p.y as usize][p.x as usize] = val.clone();
    self
  }

  pub fn at(&self, p: &Point) -> &String {
    &self.data[p.y as usize][p.x as usize]
  }

  pub fn at_checked(&self, p: &Point) -> Option<&String> {
    if p.y < 0 || p.y >= self.height || p.x < 0 || p.x >= self.width { return None }
    Some(&self.data[p.y as usize][p.x as usize])
  }

  pub fn points(&self) -> Vec<(Point, String)> {
    let mut ps = Vec::with_capacity(self.data.len() * self.data[0].len());
    for (y, row) in self.data.iter().enumerate() {
      for (x, val) in row.iter().enumerate() {
        ps.push((Point {x: x as i32, y: y as i32}, val.clone()));
      }
    }
    ps
  }

  pub fn neighbors(&self, coord: Point, adj: impl Adjacency) -> Vec<Neighbor> {
    if coord.x < 0 || coord.x >= self.width || coord.y < 0 || coord.y >= self.height {
      panic!("x or y is out of bounds!");
    }
    adj.relative_locations().map(|location| {
      Neighbor {
        location: location.clone(),
        values: points_for_location(&coord, &(location.clone()))
                  .into_iter()
                  .filter_map(|point| {
                    if point.x < 0 || point.y < 0 || point.x >= self.width || point.y >= self.height {
                      None
                    } else {
                      Some((&self.data[point.y as usize][point.x as usize]).clone())
                    }
                  })
                  .collect()
      }
    }).collect()
  }

  pub fn raycast(&self, origin: &Point, dir: &Vector, mut pred: impl FnMut(&Point, &String) -> bool) -> Option<Point> {
    let mut step = Point { x: origin.x, y: origin.y };
    loop {
      let next_step = Point { x: step.x + dir.x, y: step.y + dir.y };
      if next_step.x >= self.width || next_step.x < 0 || next_step.y < 0 || next_step.y >= self.height { return None }
      if pred(&next_step, self.at(&next_step)) { return Some(step) }
      step = next_step;
    }
  }

  pub fn print(&self) {
    for row in &self.data {
      println!("{}", row.join(""))
    }
  }
}


#[derive(Clone, Debug)]
pub enum SplitResult {
  Result(Vec<SplitResult>),
  Value(String)
}

// Recursively split a string, pulling delimiters off the input delimiters list from the back
pub fn supersplit(input: &str, delims: &mut Vec<&str>) -> SplitResult {
  let result: SplitResult;
  if delims.is_empty() {
    result = SplitResult::Value(input.to_string());
  } else {
    let mut output = vec![];
    match delims.pop() {
      Some(delim) => {
        for part in input.split(delim) {
          output.push(supersplit(part, &mut delims.to_vec()));
        }
      }
      None => {}
    }

    result = SplitResult::Result(output);
  }
  result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supersplit() {
      let result = supersplit("some crazy,string bro", &mut vec![" ", ","]);
      let matches = match result {
        SplitResult::Result(r) => match &r[..] {
          [SplitResult::Result(first), SplitResult::Result(second)] => match [&first[..], &second[..]] {
            [[SplitResult::Value(x), SplitResult::Value(y)], [SplitResult::Value(z), SplitResult::Value(zz)]] => {
              //println!("OUT = {:?}", [x, y, z, zz]);
              [x, y, z, zz] == ["some", "crazy", "string", "bro"]
            },
            _ => false,
          },
          _ => false,
        },
        SplitResult::Value(_) => false,
      };
      assert_eq!(matches, true);
    }
}