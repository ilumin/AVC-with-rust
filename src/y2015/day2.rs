pub fn get_area(input: &str) -> (isize, isize) {
  // split string and parse to Vec<isize>
  let mut parse_input: Vec<isize> = input.split("x").map(|s| s.parse().unwrap()).collect();

  // sorted Vector
  parse_input.sort();

  // matched
  let w = parse_input[0];
  let h = parse_input[1];
  let l = parse_input[2];
  
  let answer1 = (2*l*w) + (2*w*h) + (2*h*l) + (w*h);
  let answer2 = (w+w+h+h) + (w*h*l);

  (answer1, answer2)
}

#[cfg(test)]
mod tests {
  use super::*;
  use avc::read;

  #[test]
  fn validate() {
    assert_eq!(get_area("2x3x4"), (58, 34));
    assert_eq!(get_area("1x1x10"), (43, 14));
  }

  #[test]
  fn part_1() {
    let mut answer1: isize = 0;
    let mut answer2: isize = 0;
    let input = read("./src/y2015/day2.txt");
    
    for s in input.lines() {
      let (a, b) = get_area(s);
      answer1 += a;
      answer2 += b;
    }

    println!("answer1: {}", answer1);
    println!("answer2: {}", answer2);
  }
}
