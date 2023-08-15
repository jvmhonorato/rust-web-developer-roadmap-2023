#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

// fn main() {
//     println!("what is ur name?");
//     let mut name:String = String::new();
//     let greeting: &str = "Nice to meet u!!";
//     io::stdin().read_line(&mut name)  
//     .expect("Didn't Receive Input");
// println!("Hello, {}! {}", name.trim_end(), greeting);
// }

fn main() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age: &str = "47";
    let mut age: u32 = age.trim().parse()
    .expect("Age wasn't assigned a number");
age = age + 1;
println!("I'm {} and i want ${}", age, ONE_MIL);
}

