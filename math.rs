use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};

use std::io::stdin;

fn main() {
  println!("5 + 4 = {}", 5 + 4);
  println!("5 - 4 = {}", 5 - 4);
  println!("5 * 4 = {}", 5 * 4);
  println!("5 / 4 = {}", 5 / 4);
  println!("5 % 4 = {}", 5 % 4);
  
  let mut neg_4 = -4i32;
  
  println!("abs(-4) = {}", neg_4.abs());
  println!("4 ^ 6 = {}", 4i32.pow(6));
  println!("sqrt 9 = {}", 9f64.sqrt());
  println!("cbrt 9 = {}", 27f64.cbrt());

  println!("round 1.45 = {}", 1.45f64.round());
  println!("floor 1.45 = {}", 1.45f64.floor());
  println!("ceiling 1.45 = {}", 1.45f64.ceil());
  println!("e ^ 2 = {}", 2f64.exp());

  println!("log(2) = {}", 2f64.ln());
  println!("log10(2) = {}", 2f64.log10());
  println!("90 to randians = {}", 90f64.to_radians());
  println!("PI to degrees = {}", 3.14f64.to_degrees());

  println!("Max 4, 5 = {}", 5f64.max(5f64));
  println!("Min 4, 5 = {}", 4f64.min(5f64));
  
  println!("Sin 3.14 = {}", 3.14f64.sin());

}


