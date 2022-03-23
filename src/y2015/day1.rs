pub fn answer(input: &str) -> i32 {
  // let input = avc::read("./src/y2015/day1.txt");

  // find and count charactor 
  
  return 3;
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::read;

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

  fn get_answer() {
    let input = read("./src/y2015/day1.txt");
    assert_eq!(answer(input), 3);
  }
}