extern crate project7;
use project7::prob1_basics::*;  // import all problem 1 functions


#[test]
fn test_gauss1() {
  //           left  right
  //         EXPECT  ACTUAL
  assert_eq!(1     , gauss(1)  );
  assert_eq!(15    , gauss(5)  );
  assert_eq!(55    , gauss(10) );
  assert_eq!(190   , gauss(19) );
}

#[test]
fn test_gauss2() {
  //           left  right
  //         EXPECT  ACTUAL
  assert_eq!(-1    , gauss(-2) ); // check negative handling
  assert_eq!(-1    , gauss(-400));
  assert_eq!(54615 , gauss(330));
}

#[test]
fn test_in_range1() {
  //         left  right
  //       EXPECT  ACTUAL
  assert_eq!( 3, in_range(&[5,2,1,3,9], 2, 5) );
  assert_eq!( 1, in_range(&[5,2,1,3,9], 3, 4) );
  assert_eq!( 4, in_range(&[5,2,1,3,9], 2, 10));
  assert_eq!( 1, in_range(&[1,3,5], 2, 4)     );
  assert_eq!( 2, in_range(&[1,2,3,5,6], 2, 4) );
}

#[test]
fn test_in_range2() {
  //         left  right
  //       EXPECT  ACTUAL
  assert_eq!( 0, in_range(&[], 2, 10)         );              // check empty handling
  assert_eq!( 6, in_range(&[-4,-3,-2,-1,0,1,2,3,4], -3, 2) ); // ensure negatives handled
  let v100 : Vec<i32> = (0..100).collect();                   // 0,1,2,...99
  assert_eq!(26, in_range(&v100, 25, 50) );                   // larger range check

}

#[test]
fn test_subset1() {
  //          left  right
  //        EXPECT  ACTUAL
  assert_eq!(true , subset(&[1,3,2],           &[1,2,3,4,5])       ); 
  assert_eq!(false, subset(&[1,3,2],           &[1,3,4,5])         ); 
  assert_eq!(true , subset(&["a","c","d","c"], &["d","c","a"])     ); 
  assert_eq!(true , subset(&["a","c","d","c"], &["d","c","a","r"]) ); 
  assert_eq!(false, subset(&["a","q","d"],     &["d","c","a","r"]) ); 
}

#[test]
fn test_subset2() {
  //          left  right
  //        EXPECT  ACTUAL
  assert_eq!(false, subset(&[1,3,2],           &[])                 ); 
  assert_eq!(false, subset(&["a","b","d","e"], &[])                 ); 
  assert_eq!(true,  subset(&[],                &[1,3,4,5])          ); 
  assert_eq!(true , subset(&["a","c","d","c"], &["d","c","a"])      ); 

  assert_eq!(true,  subset::<i32>(&[],         &[])                 ); // since both arrays are empty, compiler needs a hint to infer the types of the arrays
}


#[test]
fn test_mean1() {
  //               left       right
  //              EXPECT      ACTUAL
  assert_eq!( Some(10.5)    , mean(&[10.0, 5.0, 7.0, 20.0]) );
  assert_eq!( Some(-5.0/3.0), mean(&[-10.0, 3.0, 2.0])      );
  //               -1.66667
}

#[test]
fn test_mean2() {
  let v50 : Vec<f64> = (-50..=50).map(|x| x as f64).collect(); 
  // -50.0,-49.0,-48.0...-1.0,0.0,1.0,...50.0

  //               left       right
  //              EXPECT      ACTUAL
  assert_eq!( None          , mean(&[])                     );
  assert_eq!( Some(0.0)     , mean(&v50)                    );
}

#[test]
fn test_to_binstring1() {
  //               left       right
  //              EXPECT      ACTUAL
  assert_eq!(        "1", to_binstring(    1) );
  assert_eq!(       "10", to_binstring(    2) );
  assert_eq!(       "11", to_binstring(    3) );
  assert_eq!(      "110", to_binstring(    6) );
  assert_eq!(     "1001", to_binstring(    9) );
  assert_eq!(   "100000", to_binstring(   32) );
  assert_eq!(   "100011", to_binstring(   35) );
}

#[test]
fn test_to_binstring2() {
  //               left           right
  //              EXPECT          ACTUAL
  assert_eq!(                "0", to_binstring(    0) ); // special case of 0
  assert_eq!(        "111111111", to_binstring(  511) );
  assert_eq!(       "1110000100", to_binstring(  900) );
  assert_eq!(      "10000000000", to_binstring( 1024) );
  assert_eq!(      "10001001011", to_binstring( 1099) );
  assert_eq!( "1010001111100000", to_binstring(41952) );
  assert_eq!("10010101000111111", to_binstring(76351) );
}


#[test]
fn test_count_words1() {
  // checks basic functionality of single-space separated words 

  //      left           right
  //     EXPECT          ACTUAL
  assert_eq!( 1, count_words( &String::from("hello")) );
  assert_eq!( 2, count_words( &String::from("hello world")) );
  assert_eq!( 5, count_words( &String::from("every good boy does fine")) );
  assert_eq!(20, count_words( &String::from("1 2 3 4 5 6 7 8 911 10 11 12 13 14 15 16 17 18 19 20")));
}

#[test]
fn test_count_words2() {
  // checks more special cases e.g. empty string, leading / trailing
  // whitespace, multiple spaces between words, embedded tabs
  // newlines, etc.

  //      left           right
  //     EXPECT          ACTUAL
  assert_eq!( 0, count_words( &String::from("")) );
  assert_eq!( 0, count_words( &String::from("       ")) );
  assert_eq!( 1, count_words( &String::from("  SPACEY    ")) );
  assert_eq!( 2, count_words( &String::from("  wide        separation  ")) );
  assert_eq!( 3, count_words( &String::from("again,  wide        separation   ")) );
  assert_eq!( 6, count_words( &String::from("ALL ... NON - whitespace !! ")) );
  assert_eq!( 7, count_words( &String::from("tabs\tor spaces\tor\ttabs\tor spaces")));
  assert_eq!( 4, count_words( &String::from("there\nis\nanother\nline\n")));
  assert_eq!(21, count_words( &String::from("I saw the best minds\tof my .... generation\nDESTROYED  \t  \n by madness , , , starving !! hysterical ?? naked, ...") ));
}

// create a string representation of the matrices for testing
// circulant() function
fn mat_string<T>(mat: &Vec<Vec<T>>) -> String
  where T:std::fmt::Display
{
  let mut width = 0;            // calculate maximum width of any element
  for row in mat {              // by iterating over it
    for x in row {
      let w = x.to_string().len();
      width = std::cmp::max(width,w);
    }
  }
  let mut str = String::new();  // string to build matrix
  for row in mat {              // iterate over each row/elem adding
    for x in row {              // to the string
      let x_str = format!("{:width$} ",x,width=width);
      str.push_str(&x_str);
    }
    str.push_str(&"\n");
  }
  return str;
}

// create a string representation of a vector for testing circulant()
// function
fn vec_string<T>(row: &Vec<T>) -> String
  where T:std::fmt::Display
{
  let mut width = 0;            // calculate maximum width of any element
  for x in row {
    let w = x.to_string().len();
    width = std::cmp::max(width,w);
  }
  let mut str = String::new();  // string to build matrix
  str.push_str(&"[");
  for x in row {              // to the string
    let x_str = format!("{:width$} ",x,width=width);
    str.push_str(&x_str);
  }
  str.push_str(&"]");
  return str;

}

// create an error message for circulant tests
fn error_message<T>(input: &Vec<T>, expect: &Vec<Vec<T>>, actual: &Vec<Vec<T>>) -> String
  where T:std::fmt::Display
{
  let expect_str = mat_string(&expect);
  let actual_str = mat_string(&actual);
  let mut msg = String::new();
  msg.push_str(&format!("Expected and Actual do not match\n"));
  msg.push_str(&format!("circulant({})\n",vec_string(&input)));
  msg.push_str(&format!("expect:\n{}\nactual:\n{}",expect_str,actual_str));
  return msg;
}  

#[test]
fn test_circulant1() {
  let input = vec![1,2,3];
  let expect = vec![
    vec![1,2,3],
    vec![2,3,1],
    vec![3,1,2],
  ];
  let actual = circulant(&input);
  assert!(expect == actual, "{}", error_message(&input,&expect,&actual));
}

#[test]
fn test_circulant2() {
  let input =  vec!["a","b","c","d"];
  let expect = vec![
    vec!["a","b","c","d"],
    vec!["b","c","d","a"],
    vec!["c","d","a","b"],
    vec!["d","a","b","c"],
  ];
  let actual = circulant(&input);
  assert!(expect == actual, "{}", error_message(&input,&expect,&actual));
}

#[test]
fn test_circulant3() {
  let input = vec!["**","--","//","%%","@@"];
  let expect = vec![
    vec!["**", "--", "//", "%%", "@@"], 
    vec!["--", "//", "%%", "@@", "**"], 
    vec!["//", "%%", "@@", "**", "--"], 
    vec!["%%", "@@", "**", "--", "//"], 
    vec!["@@", "**", "--", "//", "%%"], 
  ];
  let actual = circulant(&input);
  assert!(expect == actual, "{}", error_message(&input,&expect,&actual));
}

#[test]
fn test_circulant4() {
  let input: Vec<i32> = ((10..=100).step_by(10)).collect(); // [10 20 30 40 .. 100]
  let expect = vec![
    vec![ 10,  20,  30,  40,  50,  60,  70,  80,  90, 100,], 
    vec![ 20,  30,  40,  50,  60,  70,  80,  90, 100,  10,], 
    vec![ 30,  40,  50,  60,  70,  80,  90, 100,  10,  20,], 
    vec![ 40,  50,  60,  70,  80,  90, 100,  10,  20,  30,], 
    vec![ 50,  60,  70,  80,  90, 100,  10,  20,  30,  40,], 
    vec![ 60,  70,  80,  90, 100,  10,  20,  30,  40,  50,], 
    vec![ 70,  80,  90, 100,  10,  20,  30,  40,  50,  60,], 
    vec![ 80,  90, 100,  10,  20,  30,  40,  50,  60,  70,], 
    vec![ 90, 100,  10,  20,  30,  40,  50,  60,  70,  80,], 
    vec![100,  10,  20,  30,  40,  50,  60,  70,  80,  90,], 
  ];
  let actual = circulant(&input);
  assert!(expect == actual, "{}", error_message(&input,&expect,&actual));
}
