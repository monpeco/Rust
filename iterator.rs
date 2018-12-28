use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};

use std::io::stdin;

fn main() {
  let rand_string = "I am a random string";
  
  let count = rand_string.chars().count();
  let mut chars = rand_string.chars();
  
  let mut indiv_char = chars.next();
  
  loop{
    match indiv_char{
      Some(x) => println!("{}", x),
      None => break,
    }
    indiv_char = chars.next();
  }
  
}


