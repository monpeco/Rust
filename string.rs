use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};

use std::io::stdin;

fn main() {
  let rand_string = "I am a random string";
  
  println!("length: {}", rand_string.len());
  
  // Split a string in half at index
  let(first, second) = rand_string.split_at(6);
  println!("first: [{}], second: [{}]", first, second);
  
}


