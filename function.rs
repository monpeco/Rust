use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};

use std::io::stdin;

fn main() {
  say_hello("World!!");
}

fn say_hello(name: &str){
  println!("Hello {}", name);
}


