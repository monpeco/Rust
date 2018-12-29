use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};

use std::io::stdin;

fn main() {
  let rand_tuple = ("Derek", 40);
  
  let rand_tuple_2: (&str, i8) = ("Derek", 40);
  
  println!("Name: {}", rand_tuple_2.0);
  println!("age: {}", rand_tuple_2.1);
}


