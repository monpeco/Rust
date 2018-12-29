use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};

use std::io::stdin;

fn main() {
  let mut vect1 = vec![1,2,3,4,5];
  println!("Item 2: {}", vect1[1]);
  
  for i in &vect1{
    println!("vect1: {}",i);
  }

  vect1.push(6);   // add at the end

  vect1.pop();   // remove from the end

}


