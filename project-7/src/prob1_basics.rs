use regex::Regex;  // for count_words() function
// prob1_basics.rs 

// To get the tests up and running fast, you may wish to add all
// functions with empty bodies / dummy returns intitially

/// Returns the sum 1 + 2 + ... + n
/// 
/// If n is less than 0, return -1
/// 
/// May use either a fixed equation to calculate the answer or an
/// iterative approach (though Gauss himself would likely prefer the
/// former...)
///
/// EXAMPLES:
/// gauss(5)  -> 15
/// gauss(10) -> 55
/// gauss(-2) -> -1
pub fn gauss(n: i32) -> i32 {
  if n < -1 {
    -1
  } else {
    (n * (n + 1)) / 2
  }
}


/// Returns a count of elements in `slice` of i32that satisfy:
///   lo <= x <= hi
/// Uses an iterator over the slice to ensure all elements are visited.
///
/// EXAMPLES:
/// in_range([5,2,1,3,9], 2, 5)  -> 3
/// in_range([5,2,1,3,9], 3, 4)  -> 1
/// in_range([5,2,1,3,9], 2, 10) -> 4
/// in_range([], 2, 10)          -> 0
pub fn in_range(slice: &[i32], lo: i32, hi: i32) -> i32 {
  slice.iter().filter(|&&x| x >= lo && x <= hi).count() as i32
}

/// Calculates and returns the mean of elements in `slice` of floating
/// point values. To handle empty slices, this function returns an
/// Option: None for empty slices, and Some(mean) for filled slices.
///
/// EXAMPLES:
/// mean([])                     -> None
/// mean([10.0, 5.0, 7.0, 20.0]) -> Some(10.5)
/// mean([-10.0, 3.0, 2.0])      -> Some(-1.6666)
pub fn mean(slice: &[f64]) -> Option<f64> {
  if slice.is_empty() {
      None
  } else {
      let sum: f64 = slice.iter().sum();
      let count = slice.len() as f64;
      Some(sum / count)
  }
} 

/// Returns true if `slicea` is a subset of `sliceb` and false
/// otherwise. The function is generic so works with slices of any
/// type. The slices are not ordered so must be searched sequentially.
/// 
/// EXAMPLES:
///   subset([1,3,2], [1,2,3,4,5]) -> true
///   subset([1,3,2], [1,3,4,5])   -> false
///   subset(["a","c","d","c"], ["d","c","a"])     -> true
///   subset(["a","c","d","c"], ["d","c","a","r"]) -> true
///   subset(["a","q","d"],     ["d","c","a","r"]) -> false
///
/// NOTE: Utilizing certain methods of slices may lead to shorter code
/// by letting methods detect whether elements are contained or not.
pub fn subset<T>(slicea: &[T], sliceb: &[T]) -> bool
  where T: PartialEq<T>         // to compare elements via ==, type must implement PartialEq Trait
  {
    slicea.iter().all(|item_a| sliceb.contains(item_a))
  }

/// Return a string showing the binary digits of the given unsigned
/// (positive) integer. Calculates the binary digits through repeated
/// division by 2 where the remainders become the digits in the binary
/// number. A survey of this method is here
/// https://www.cuemath.com/numbers/decimal-to-binary/
///
/// While converting, pushes digits of "0" or "1" into a container
/// type, vector being a good choice.  These digits need to be
/// reversed at the end of the computation (last digit found is most
/// significant, leftmost bit) so visits the digits in reverse order
/// in the container appending them to a String which is ultimately
/// returned.
///
/// No leading zeros are provided so the left-most character is always
/// a 1 EXCEPT in the special case of decimal value 0 which should
/// return the string "0".
/// 
/// EXAMPLES:
/// to_binstring(  0) ->         "0"
/// to_binstring(  2) ->        "10"
/// to_binstring(  9) ->      "1001"
/// to_binstring( 32) ->    "100000"
/// to_binstring(510) -> "111111110"
pub fn to_binstring(num: u32) -> String {
  if num == 0 {
    return "0".to_string();
  }

  let mut num = num;
  let mut binary_digits = Vec::new();

  while num > 0 {
      let remainder = num % 2;
      binary_digits.push(remainder);
      num /= 2;
  }

  binary_digits.reverse();
  binary_digits.iter().map(|&digit| if digit == 1 { '1' } else { '0' }).collect()
}


/// Construct a circulant matrix (2D vector) from the given
/// slice. Briefly a circulant matrix has row i rotated left by i
/// elements with row 0 being identical to the parameter r0_slice. A
/// vector or rows is allocated and each row is constructd as a
/// rotation left by i indices from the 0th row. The function is
/// generic and accepts any type that can be Cloned elements from
/// r0_slice must have clone() invoked on them to get a memory
/// distinct copy of them.
///
/// EXAMPLES:
/// 
/// circulant(&[1,2,3]) ->
///  [[1,2,3],
///   [2,3,1],
///   [3,1,2]]
///
/// circulant(&["a","b","c","d"]) ->
///  [["a","b","c","d"]
///   ["b","c","d","a"]
///   ["c","d","a","b"]
///   ["d","a","b","c"]]
pub fn circulant<T>(r0_slice: &[T]) -> Vec<Vec<T>>
  where T:Clone                 // elements implement Clone so have .clone()
{
    let n = r0_slice.len();
    (0..n)
        .map(|i| {
            r0_slice.iter().cycle().skip(i).take(n).cloned().collect()
        })
        .collect()
}

/// Returns a count of the number of "words" in `text`. The notion of
/// a word is a series of non-whitespace characters, (whitespace is
/// space, tab, newline). Regular expressions may be used for counting
/// words or character-by-character iteration to count whitespace
/// transitions. In either case, iteration through the string will be
/// required, either by regex match by direct iteration on the
/// characters.
/// 
/// NOTE: The Regular expression "crate" is marked as a dependency for
/// this project so will be downloaded HOWEVER you will need to add an
/// appropriate "use" statement to access its functionality here.
/// 
/// EXAMPLES:
/// count_words( &String::from("hello world"))                          -> 2
/// count_words( &String::from("       "))                              -> 0
/// count_words( &String::from("ALL ... NON - whitespace !! "))         -> 6
/// count_words( &String::from("tabs\tor spaces\tor\ttabs\tor spaces")) -> 7
pub fn count_words(text: &String) -> i32 {
  let re = Regex::new(r"\S+").unwrap();
  re.find_iter(text).count() as i32
}
