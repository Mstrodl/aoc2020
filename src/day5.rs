use aoc_runner_derive::{aoc, aoc_generator};

pub struct Pass {
  x: usize,
  y: usize,
  seatId: usize
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<Pass> {
  let mut passes: Vec<Pass> = Vec::new();

  for line in input.split("\n") {
    let mut y = 0;
    let mut min = 0;
    let mut max = 127;
    let mut front = false;
    for c in line.chars() {
      if !front && (c == 'L' || c == 'R') {
        front = true;
        y = min;
        max = 7;
        min = 0;
      }

      let delta: usize = (max + 1 - min) / 2;

      match c {
        'F' | 'L' => {
          max -= delta;
        }
        'B' | 'R' => {
          min += delta;
        }
        _ => panic!("Fuck!")
      };
    }

    let x = min;

    assert_eq!(min, max);

    println!("X: {} Y: {}", x, y);
    let pass = Pass { x: x, y: y, seatId: ((y * 8) + x) };
    passes.push(pass);
  }

  return passes;
}

#[aoc(day5, part1)]
pub fn part1(passes: &Vec<Pass>) -> usize {
  let mut max = 0;
  println!("Passes: {}", passes.len());
  for pass in passes {
    if pass.seatId > max {
      max = pass.seatId;
    }
  }
  return max;
}

#[aoc(day5, part2)]
pub fn part2(passes: &Vec<Pass>) -> usize {
  let mut seats = [[false; 128]; 8];

  for pass in passes {
    seats[pass.x][pass.y] = true;
  }

  let mut foundY = false;
  'yLoop: for y in 0..128-2 {
    for x in 0..8 {
      if foundY {
        if seats[x][y] == false {
          return x + (y*8);
        }
      } else if seats[x][y] == false {
        continue 'yLoop;
      }
    }
    if !foundY {
      println!("Entire row checks out?");
      foundY = true;
    }
  }
  panic!("Missing seat?");
}
