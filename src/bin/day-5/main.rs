#[macro_use]
extern crate lazy_static;
mod models;

use advent::read_input_for_day;
use models::graph::Graph;
use models::line::Line;
use std::io;

fn run(part: i8, include_diagonals: bool, input: Vec<Line>) -> io::Result<()> {
  let mut graph = Graph::new(include_diagonals);
  for line in input {
    graph.chart(line);
  }

  println!("part {}:", part);
  println!(
    "    At least two lines overlap in {} points.",
    graph.n_overlapping_points()
  );
  Ok(())
}

fn main() -> io::Result<()> {
  run(1, false, read_input_for_day::<Line>(5)?)?;
  run(2, true, read_input_for_day::<Line>(5)?)?;

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::str::FromStr;

  fn get_lines() -> Vec<Line> {
    vec![
      "0,9 -> 5,9",
      "8,0 -> 0,8",
      "9,4 -> 3,4",
      "2,2 -> 2,1",
      "7,0 -> 7,4",
      "6,4 -> 2,0",
      "0,9 -> 2,9",
      "3,4 -> 1,4",
      "0,0 -> 8,8",
      "5,5 -> 8,2",
    ]
    .into_iter()
    .map(|s| Line::from_str(s).unwrap())
    .collect()
  }

  #[test]
  fn part_one() {
    let mut graph = Graph::new(false);
    for line in get_lines() {
      graph.chart(line);
    }

    assert_eq!(graph.n_overlapping_points(), 5)
  }

  #[test]
  fn part_two() {
    let mut graph = Graph::new(true);
    for line in get_lines() {
      graph.chart(line);
    }

    assert_eq!(graph.n_overlapping_points(), 12)
  }
}
