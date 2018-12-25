use std::{i8, i16, i32, i64, u8, u16, u32, u64, isize, usize, f32, f64};

use std::io::stdin;

fn main() {
    let is_it_true: bool = true;
    let let_x: char = 'x';
    
    println!("It is {0} that {1} is {0}", is_it_true, let_x);
    
    //println!("{:.2}", 1.234);
    
    // print information in binary, hexadecimal and octal
    println!("B: {:b}, H: {:X}, O: {:o}", 10,10,10);
    
    // print with name variables and white space
    println!("ten: {ten:>ws$}, one: {one}", ten=10, one=1, ws=5);
    
    // print witn zeros
    println!("ten: {ten:>0ws$}", ten=10, ws=8);
}