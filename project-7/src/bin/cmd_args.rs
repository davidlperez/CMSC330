// cmd_args.rs: demonstrate accessing command line arguments
// in a main function

use std::env;

fn main() {

  for (idx,arg) in env::args().enumerate() {
    println!("arg[{idx}]: {arg}");
  }

  // collects all args into a single vector, not needed for the above
  // funcionality but useful as a demo of how to do so
  let vec_args: Vec<String> = env::args().collect();
  println!("{} total args",vec_args.len());
}
