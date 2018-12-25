use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};

use std::io::stdin;

fn main() {

  // loop
  let mut x = 1;
  
  loop{
    // if even, print number
    if((x % 2) == 0){
      println!("{}", x);
      x += 1;
      continue;
    }
    if(x > 10){
      break;
    }
    x += 1;
    continue;
  }

  // while
  let mut y = 1;
  
  while y <= 10{
    println!("While : {}", y);
    y += 1;
  }
  
  // for range
  for z in 1..10 {  // `..=` for an inclusive range
    println!("z: {}", z);
  }
  
}


