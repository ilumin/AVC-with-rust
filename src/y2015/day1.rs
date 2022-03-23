pub fn answer(input: &str) -> isize {
  let total_size = input.chars().count() as isize;
  let down_size = input.replace("(", "").chars().count() as isize;
  let up_size = total_size - down_size;
  let direction = up_size - down_size;

  direction.try_into().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;
  use avc::read;

  #[test]
  fn validate() {
    assert_eq!(answer("(())"), 0);
    assert_eq!(answer("()()"), 0);
    assert_eq!(answer("((("), 3);
    assert_eq!(answer("(()(()("), 3);
    assert_eq!(answer("))((((("), 3);
    assert_eq!(answer("())"), -1);
    assert_eq!(answer("))("), -1);
    assert_eq!(answer(")))"), -3);
    assert_eq!(answer(")())())"), -3);
  }

  #[test]
  fn get_answer() {
    let input = read("./src/y2015/day1.txt");
    assert_eq!(answer(&input), 232);
  }
}