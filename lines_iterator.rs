use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};

use std::io::stdin;

fn main() {
  let rand_string = "I am a random string\nThere are other stings like it\nThis string";
  
  let mut iter = rand_string.lines();
  
  let mut indiv_word = iter.next();
  
  loop{
    match indiv_word {
      Some(x) => println!("{}", x),
      None => break,
    }
    indiv_word = iter.next();
  }
  
}


