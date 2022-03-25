use std::collections::HashMap;

fn counter(input: Option<&isize>) -> isize {
  match input {
    Some(&number) => number + 1,
    _ => 1
  }
}

pub fn show_me_the_answer(input: &str) -> (usize, usize) {
  let (mut x, mut y): (isize, isize) = (0, 0);
  let (mut xz, mut yz): (isize, isize) = (0, 0);
  let (mut ps, mut pr): ([isize; 2], [isize; 2]) = ([0, 0], [0, 0]);
  let mut houses: HashMap<(isize, isize), isize> = HashMap::new();
  let mut housesz: HashMap<(isize, isize), isize> = HashMap::new();

  // first position should count
  houses.insert((x, y), 1);
  housesz.insert((xz, yz), 1);
  
  // loop all char from input
  for (index, c) in input.chars().enumerate() {
    println!("got: {}", c);

    let pi = index % 2;
    
    // then match with > < ^ v
    let ((a, b), (az, bz)) = match c {
      '>' => ((x+1, y), (ps[pi]+1, pr[pi])),
      '<' => ((x-1, y), (ps[pi]-1, pr[pi])),
      '^' => ((x, y+1), (ps[pi], pr[pi]+1)),
      'v' => ((x, y-1), (ps[pi], pr[pi]-1)),
      _ => ((x, y), (ps[pi], pr[pi])),
    };

    // reassign position
    // note: we can use detructure assignment on nightly build 
    // with `#![feature(destructuring_assignment)]` on `main.rs`
    // but I don't know how to set Rust version on Replit
    // so I leave it with basic assignment here
    x = a;
    y = b;
    xz = az;
    yz = bz;
    ps[pi] = az;
    pr[pi] = bz;

    println!("turn {} move: ({}, {})", pi, xz, yz);

    // store all the position in Hashmap 
    houses.insert(
      (x, y),
      counter(houses.get(&(x, y)))
    );
    housesz.insert(
      (xz, yz),
      counter(housesz.get(&(xz, yz)))
    );
  }

  (
    houses.keys().len(),
    housesz.keys().len()
  )
}

#[cfg(test)]
mod tests {
  use super::*;
  use avc::read;

  // #[test]
  fn validate() {

    // let mut houses = HashMap::new();
    // houses.insert((0,0), 1);
    // houses.insert((0,1), 2);
    // test on counter
    // assert_eq!(counter(houses.get(&(0,0))), 2);
    // assert_eq!(counter(houses.get(&(0,1))), 3);
    // assert_eq!(counter(houses.get(&(1,1))), 1);
    
    assert_eq!(show_me_the_answer(">"), (2, 2));
    assert_eq!(show_me_the_answer("^>v<"), (4, 3));
    assert_eq!(show_me_the_answer("^v^v^v^v^v"), (2, 11));
  }

  #[test]
  fn part_1() {
    let mut answer1: usize = 0;
    let mut answer2: usize = 0;
    let input = read("./src/y2015/day3.txt");

    assert_eq!(show_me_the_answer(&input), (2592, 2360));
  }
}
