use std::fs::File;
use std::io::Read;

pub fn read(filename: &str) -> String {
  match File::open(filename) {
    Ok(mut file) => {
      let mut content = String::new();
      file.read_to_string(&mut content).unwrap();
      content
    }
    Err(error) => {
      println!("Error opening file {}: {}", filename, error);
      String::from("")
    }
  }
}