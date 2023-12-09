// readfile.rs: demonstrates a couple patterns to read files line by
// line. Reads from the gettsyburg.txt file and just prints each line
// prepended with a number.
//
// Try changing the file name to one that doesn't exist to see the
// errors pop up. May need to comment out some function calls to
// get to how the later ones work.

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main(){
  let fname = "test-data/gettysburg.txt";

  println!("readlines1():");
  readlines1(&fname);
  println!("");

  println!("readlines2():");
  readlines2(&fname);
  println!("");

  println!("readlines3():");
  let _ = readlines3(&fname);   // needed to suppress "unused value" warning 
}


// Explicit version wherein each time an I/O operation returns a
// possible Error, that error is matched against the relevant Variant
// types. This style is clear but verbose and cumbersome.
fn readlines1(fname: &str) {
  let file = match File::open(&fname) {
    Err(why) => panic!("Couldn't open {fname}: {}",why),    // annoyingly format sub with {why} doesn't work
    Ok(file) => file
  };
  let reader = BufReader::new(file);
  let mut linum = 1;
  for line in reader.lines() {
    let text = match line {
      Err(why) => panic!("Couldn't read line {linum}: {}",why),
      Ok(text) => text
    };
    println!("{}: {}", linum, text);
    linum += 1;
  }
}

// Modified version that omits the pattern matching in favor of the
// .expect(errmsg) function which has a similar effect to the above
// pattern matching: return the Ok(..) datum or panic on an Err
fn readlines2(fname: &str) {
  let file = File::open(&fname).expect(&format!("Couldn't open file {fname}"));
  let reader = BufReader::new(file);
  let mut linum = 1;
  for line in reader.lines() {
    let text = line.expect("Couldn't read line");
    println!("{}: {}", linum, text);
    linum += 1;
  }
}


// This version uses the schwank ? operator which further cuts down
// boilerplate. If the expression preceding ? (like
// 'do_something(arg)?') results in an Err, the resulting Err is
// returned from the function. This requires altering the return type
// as rather than immediately panic! on an Err, the error is kicked up
// to the calling function.
// 
// from: https://stackoverflow.com/questions/45882329/read-large-files-line-by-line-in-rust
fn readlines3(fname: &str) -> io::Result<()> {
  let file = File::open(&fname)?;
  let reader = BufReader::new(file);
  let mut linum = 1;
    for line in reader.lines() {
      println!("{}: {}", linum, line?);
      linum += 1;
    }

  Ok(())
}


// Original deeply nested version of matching
// fn readlines1() {
//   match File::open(&fname) {
//     Err(why) => panic!("Couldn't open {}: {}", fname, why),
//     Ok(file) => {
//       let reader = BufReader::new(file);
//       let mut linum = 1;
//       for line in reader.lines() {
//         match line {
//           Err(why) => panic!("Couldn't read line {}: {}", linum, why),
//           Ok(text) => {
//             println!("{}: {}", linum, text);
//           }
//         }
//         linum += 1;
//       }
//     }
//   }
// }
