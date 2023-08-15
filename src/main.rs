#![allow(unused)]

use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main() {
    println!("what is ur name?");
    let mut name:String = String::new();
    let greeting: &str = "Nice to meet u!!";
    io::stdin().read_line(&mut name)  
    .expect("Didn' t Receive Input");
}
 