fn tick(input: char) -> isize {
  match input {
    '(' => 1,
    ')' => -1,
    _ => 0
  }
}

pub fn answer(input: &str, checkBasement: bool) -> isize {
  let mut direction: isize = 0;

  for (index, c) in input.chars().enumerate() {
    direction = direction + tick(c);
    
    if direction == -1 && checkBasement {
      return index as isize;
    }
  }

  direction
}

#[cfg(test)]
mod tests {
  use super::*;
  use avc::read;

  #[test]
  fn validate() {
    assert_eq!(answer("(())", false), 0);
    assert_eq!(answer("()()", false), 0);
    assert_eq!(answer("(((", false), 3);
    assert_eq!(answer("(()(()(", false), 3);
    assert_eq!(answer("))(((((", false), 3);
    assert_eq!(answer("())", false), -1);
    assert_eq!(answer("))(", false), -1);
    assert_eq!(answer(")))", false), -3);
    assert_eq!(answer(")())())", false), -3);
  }

  #[test]
  fn part_1() {
    let input = read("./src/y2015/day1.txt");
    assert_eq!(answer(&input, false), 232);
  }

  #[test]
  fn part_2() {
    let input = read("./src/y2015/day1.txt");
    assert_eq!(answer(&input, true), 1782);
  }
}