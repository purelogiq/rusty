/*
  TODO:
  - Create main function.
  - Create input loop.
  - Create an array of 52 cards.
  - Map numbers 0-51 to suite and card.
  - Randomly select one every loop.
*/

use std::io;
use std::io::prelude::*;

fn main() {
  let stdin = io::stdin();
  for line in stdin.lock().lines() {
    println!("{}", line.unwrap());
    break;
  }
  println!("Game over")
}
