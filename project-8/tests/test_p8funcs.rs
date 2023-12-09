// test_p8funcs.rs: Unit/Integration tests for the project.
// UPDATE: Tue Dec  5 05:35:46 PM EST 2023 
// 
// NOTE ON OUTPUT: println!() is used liberally in test functions as
// on a test failure, the standard Rust testing framework shows
// standard output on test failures whereas test successes hide
// output. Printing during the tests is a good way to give more
// insight into what the test's expected and actual results are to
// enable easier debugging.

#![allow(unused_imports)]
extern crate project8;
use project8::p8_funcs::*;  // import all functions
use std::path::PathBuf;


/// Prepend the path to the test-data directory; uses the directory
/// location of the Cargo.toml file avaiable via the environment
/// variable mentioned below.
fn testdata(file: &str) -> String {
  let mut d = String::from(env!("CARGO_MANIFEST_DIR"));
  d.push_str("/test-data/");
  d.push_str(file);
  return d;
  // let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  // d.push("test-data/");
  // d.push(file);
  // return d;
}

#[test]
fn test_load_string_upper1() {
  let fname = testdata("dict-fruit.txt");
  println!("fname: {}",fname);
  let actual = load_string_upper(&fname);
  let expect = vec!["APPLE","CARROT","BANANA"];
  assert!(expect == actual);
}

#[test]
fn test_load_string_upper2() {
  let fname = testdata("dict-bass.txt");
  println!("fname: {}",fname);
  let actual = load_string_upper(&fname);
  let expect = vec!["ALL","BASE","ARE","YOUR","US"];
  assert!(expect == actual);
}

#[test]
fn test_load_string_upper3() {
  let fname = testdata("dict-empty.txt");
  println!("fname: {}",fname);
  let actual = load_string_upper(&fname);
  let expect : Vec<String> = vec![];
  assert!(expect == actual);
}

#[test]
fn test_load_string_upper4() {
  let fname = testdata("dict-google-100.txt");
  println!("fname: {}",fname);
  let actual = load_string_upper(&fname);
  let expect =
    vec!["THE", "OF", "AND", "TO", "A", "IN", "FOR", "IS", "ON", "THAT", "BY",
         "THIS", "WITH", "I", "YOU", "IT", "NOT", "OR", "BE", "ARE", "FROM",
         "AT", "AS", "YOUR", "ALL", "HAVE", "NEW", "MORE", "AN", "WAS", "WE",
         "WILL", "HOME", "CAN", "US", "ABOUT", "IF", "PAGE", "MY", "HAS",
         "SEARCH", "FREE", "BUT", "OUR", "ONE", "OTHER", "DO", "NO",
         "INFORMATION", "TIME", "THEY", "SITE", "HE", "UP", "MAY", "WHAT",
         "WHICH", "THEIR", "NEWS", "OUT", "USE", "ANY", "THERE", "SEE", "ONLY",
         "SO", "HIS", "WHEN", "CONTACT", "HERE", "BUSINESS", "WHO", "WEB",
         "ALSO", "NOW", "HELP", "GET", "PM", "VIEW", "ONLINE", "C", "E",
         "FIRST", "AM", "BEEN", "WOULD", "HOW", "WERE", "ME", "S", "SERVICES",
         "SOME", "THESE", "CLICK", "ITS", "LIKE", "SERVICE", "X", "THAN",
         "FIND"];

  println!();
  for (i,x) in actual.iter().enumerate() {
    println!("{i}: {x} {}",expect[i]);
  }
  assert!(expect == actual);
}

fn v2s(strs: Vec<&str>) -> Vec<String>{
  let mut v = vec![];
  for s in strs {
    v.push(String::from(s));
  }
  return v;
}

#[test]
fn test_mark_corrected1() {
  // check for single correction at the beginning
  let dict = v2s(vec!["APPLE","BANANA","ONION"]);              
  let input = "grape     apple  \n onion\n";
  let actual = mark_corrected(&String::from(input),&dict);
  let expect ="**GRAPE**     apple  \n onion\n";
  println!("expect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_mark_corrected2() {
  // check for single correction in middle plus mixed case
  let dict = v2s(vec!["APPLE","BANANA","ONION"]);              
  let input = "apple     Grape  \n onion\n";
  let actual = mark_corrected(&String::from(input),&dict);
  let expect ="apple     **GRAPE**  \n onion\n";
  println!("expect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_mark_corrected3() {
  // check for single correction at end plus mixed case
  let dict = v2s(vec!["APPLE","BANANA","ONION"]);              
  let input = "apple     onion  \n grAPE\n";
  let actual = mark_corrected(&String::from(input),&dict);
  let expect ="apple     onion  \n **GRAPE**\n";
  println!("expect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}


#[test]
fn test_mark_corrected4() {
  // check for multiple corrections in middle
  let dict = v2s(vec!["APPLE","BANANA","ONION"]);              
  let input  = "apple \t pear onion      Grape onion";
  let actual = mark_corrected(&String::from(input),&dict);
  let expect = "apple \t **PEAR** onion      **GRAPE** onion";
  println!("expect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_mark_corrected5() {
  // check for multiple corrections in middle
  let dict = v2s(vec!["APPLE","BANANA","ONION"]);              
  let input  = "grape pear apple carrot BANANA onion pear grape";
  let actual = mark_corrected(&String::from(input),&dict);
  let expect = "**GRAPE** **PEAR** apple **CARROT** BANANA onion **PEAR** **GRAPE**";
  println!("expect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_mark_corrected6() {
  // check specifically for non-whitespace separation such as commas,
  // semicolons, and numbers
  let dict = v2s(vec!["APPLE","BANANA","ONION"]);              
  let input  = "grape,pear,apple;;carrot[BANANA]onion1pear234grape";
  let actual = mark_corrected(&String::from(input),&dict);
  let expect = "**GRAPE**,**PEAR**,apple;;**CARROT**[BANANA]onion1**PEAR**234**GRAPE**";
  println!("expect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}


#[test]
fn test_mark_corrected_empty_input() {
  // check for correct handling of empty input
  let dict = v2s(vec!["APPLE","BANANA","ONION"]);              
  let input  = "";
  let actual = mark_corrected(&String::from(input),&dict);
  let expect = "";
  println!("expect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_mark_corrected_empty_dict() {
  // check for correct handling of empty dict
  let dict = v2s(vec![]);
  let input  = "apple \t pear onion      Grape onion";
  let actual = mark_corrected(&String::from(input),&dict);
  let expect = "**APPLE** \t **PEAR** **ONION**      **GRAPE** **ONION**";
  println!("expect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_mark_corrected_contractions() {
  // check whether contractions with single quote are handled correctly
  let dict = v2s(vec!["CAN'T","BUY","ME","LOVE","Y'ALL"]);              
  let input  = "Can't buy me Love, ya'll. Love, you cannot buy me that.";
  let actual = mark_corrected(&String::from(input),&dict);
  let expect = "Can't buy me Love, **YA'LL**. Love, **YOU** **CANNOT** buy me **THAT**.";
  println!("expect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_mark_corrected_load_string1() {
  // checks combination of load_string_upper() with mark_corrected()
  let fname = testdata("dict-bass.txt");
  println!("fname: {}",fname);
  let dict = load_string_upper(&fname);
  let input  = "All your bass are belong 2 us.";
  let actual = mark_corrected(&String::from(input),&dict);
  let expect = "All your **BASS** are **BELONG** 2 us.";
  println!("expect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_mark_corrected_load_string2() {
  // checks combination of load_string_upper() with mark_corrected()
  let fname = testdata("dict-google-100.txt");
  println!("fname: {}",fname);
  let dict = load_string_upper(&fname);
  let input  = "\nAll work and no play makes Jack a dull boy...\nAll work and no...";
  let actual = mark_corrected(&String::from(input),&dict);
  let expect = "\nAll **WORK** and no **PLAY** **MAKES** **JACK** a **DULL** **BOY**...\nAll **WORK** and no...";
  println!("expect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_mark_corrected_load_string3() {
  // checks combination of load_string_upper() with mark_corrected()
  // This test uses Rust's "raw strings" which allow embedded newlines
  // and other control characters for ease of reading in the source.

  let fname = testdata("dict-google-10000.txt");
  println!("fname: {}",fname);
  let dict = load_string_upper(&fname);
  let input  = r#"
Four score and seven years ago our fathers brought forth on this continent, a
new nation, conceived in Liberty, and dedicated to the proposition that all men
are created equal.

Now we are engaged in a great civil war, testing whether that nation, or any
nation so conceived and so dedicated, can long endure. We are met on a great
battle-field of that war. We have come to dedicate a portion of that field, as a
final resting place for those who here gave their lives that that nation might
live. It is altogether fitting and proper that we should do this.
"#;
  let actual = mark_corrected(&String::from(input),&dict);
  let expect =r#"
Four score and seven years ago our fathers brought forth on this continent, a
new nation, **CONCEIVED** in Liberty, and dedicated to the proposition that all men
are created equal.

Now we are engaged in a great civil war, testing whether that nation, or any
nation so **CONCEIVED** and so dedicated, can long **ENDURE**. We are met on a great
battle-field of that war. We have come to **DEDICATE** a portion of that field, as a
final **RESTING** place for those who here gave their lives that that nation might
live. It is **ALTOGETHER** fitting and proper that we should do this.
"#;
  println!("expect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_mark_corrected_load_string4() {
  // checks combination of load_string_upper() with mark_corrected()
  // This test uses Rust's "raw strings" which allow embedded newlines
  // and other control characters for ease of reading in the source.

  let fname = testdata("dict-google-10000.txt");
  println!("fname: {}",fname);
  let dict = load_string_upper(&fname);
  let input  = r#"
Four score and seven years ago our fathers brought forth on this continent, a
new nation, conceived in Liberty, and dedicated to the proposition that all men
are created equal.

Now we are engaged in a great civil war, testing whether that nation, or any
nation so conceived and so dedicated, can long endure. We are met on a great
battle-field of that war. We have come to dedicate a portion of that field, as a
final resting place for those who here gave their lives that that nation might
live. It is altogether fitting and proper that we should do this.

But, in a larger sense, we can not dedicate -- we can not consecrate -- we can
not hallow -- this ground. The brave men, living and dead, who struggled here,
have consecrated it, far above our poor power to add or detract. The world will
little note, nor long remember what we say here, but it can never forget what
they did here. It is for us the living, rather, to be dedicated here to the
unfinished work which they who fought here have thus far so nobly advanced. It
is rather for us to be here dedicated to the great task remaining before us --
that from these honored dead we take increased devotion to that cause for which
they gave the last full measure of devotion -- that we here highly resolve that
these dead shall not have died in vain -- that this nation, under God, shall
have a new birth of freedom -- and that government of the people, by the people,
for the people, shall not perish from the earth.

Abraham Lincoln
November 19, 1863
"#;
  let actual = mark_corrected(&String::from(input),&dict);
  let expect =r#"
Four score and seven years ago our fathers brought forth on this continent, a
new nation, **CONCEIVED** in Liberty, and dedicated to the proposition that all men
are created equal.

Now we are engaged in a great civil war, testing whether that nation, or any
nation so **CONCEIVED** and so dedicated, can long **ENDURE**. We are met on a great
battle-field of that war. We have come to **DEDICATE** a portion of that field, as a
final **RESTING** place for those who here gave their lives that that nation might
live. It is **ALTOGETHER** fitting and proper that we should do this.

But, in a larger sense, we can not **DEDICATE** -- we can not **CONSECRATE** -- we can
not **HALLOW** -- this ground. The brave men, living and dead, who **STRUGGLED** here,
have **CONSECRATED** it, far above our poor power to add or **DETRACT**. The world will
little note, nor long remember what we say here, but it can never forget what
they did here. It is for us the living, rather, to be dedicated here to the
**UNFINISHED** work which they who fought here have thus far so **NOBLY** advanced. It
is rather for us to be here dedicated to the great task remaining before us --
that from these **HONORED** dead we take increased **DEVOTION** to that cause for which
they gave the last full measure of **DEVOTION** -- that we here highly resolve that
these dead shall not have died in **VAIN** -- that this nation, under God, shall
have a new birth of freedom -- and that government of the people, by the people,
for the people, shall not **PERISH** from the earth.

Abraham Lincoln
November 19, 1863
"#;
  println!("expect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_mark_corrected_load_string5() {
  // checks combination of load_string_upper() with mark_corrected()
  // This test uses the full english dictionary and should find NO
  // corrections in the text.

  let fname = testdata("dict-full-english.txt");
  println!("fname: {}",fname);
  let dict = load_string_upper(&fname);
  let input  = r#"
Four score and seven years ago our fathers brought forth on this continent, a
new nation, conceived in Liberty, and dedicated to the proposition that all men
are created equal.

Now we are engaged in a great civil war, testing whether that nation, or any
nation so conceived and so dedicated, can long endure. We are met on a great
battle-field of that war. We have come to dedicate a portion of that field, as a
final resting place for those who here gave their lives that that nation might
live. It is altogether fitting and proper that we should do this.

But, in a larger sense, we can not dedicate -- we can not consecrate -- we can
not hallow -- this ground. The brave men, living and dead, who struggled here,
have consecrated it, far above our poor power to add or detract. The world will
little note, nor long remember what we say here, but it can never forget what
they did here. It is for us the living, rather, to be dedicated here to the
unfinished work which they who fought here have thus far so nobly advanced. It
is rather for us to be here dedicated to the great task remaining before us --
that from these honored dead we take increased devotion to that cause for which
they gave the last full measure of devotion -- that we here highly resolve that
these dead shall not have died in vain -- that this nation, under God, shall
have a new birth of freedom -- and that government of the people, by the people,
for the people, shall not perish from the earth.

Abraham Lincoln
November 19, 1863
"#;
  let actual = mark_corrected(&String::from(input),&dict);
  let expect = input;           // no corrections made, all words in dictionary
  println!("expect: {expect}\nactual: {actual}");
}

/// Simple implementation of a Corrector used for testing
struct Wrong {}
impl Corrector for Wrong {
  fn correct_word(&mut self, _word: &str) -> String{
    return String::from("!WRONG!");
  }
}

#[test]
fn test_correct_string1() {
  // check for single correction at the beginning but use a provided Corrector
  let dict = v2s(vec!["APPLE","BANANA","ONION"]);              
  let mut wc = Wrong{};
  let input = "grape     apple  \n onion\n";
  let actual = correct_string(&String::from(input),&dict,&mut wc);
  let expect ="!WRONG!     apple  \n onion\n";
  println!("expect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}


#[test]
fn test_correct_string2() {
  // check for single correction at the beginning but use a provided Corrector
  let dict = v2s(vec!["APPLE","BANANA","ONION"]);              
  let mut wc = Wrong{};
  let input  = "apple \t pear onion      Grape onion";
  let actual = correct_string(&String::from(input),&dict,&mut wc);
  let expect = "apple \t !WRONG! onion      !WRONG! onion";
  println!("expect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_correct_string3() {
  // check for single correction at the beginning but use a provided Corrector
  let dict = v2s(vec!["APPLE","BANANA","ONION"]);              
  let mut wc = Wrong{};
  let input  = "grape pear apple carrot BANANA onion pear grape";
  let actual = correct_string(&String::from(input),&dict,&mut wc);
  let expect = "!WRONG! !WRONG! apple !WRONG! BANANA onion !WRONG! !WRONG!";
  println!("expect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_correct_string4() {
  // combines correct_string() with dictionary loading
  let fname = testdata("dict-google-10000.txt");
  println!("fname: {}",fname);
  let dict = load_string_upper(&fname);
  let mut wc = Wrong{};
  let input  = r#"
Four score and seven years ago our fathers brought forth on this continent, a
new nation, conceived in Liberty, and dedicated to the proposition that all men
are created equal.

Now we are engaged in a great civil war, testing whether that nation, or any
nation so conceived and so dedicated, can long endure. We are met on a great
battle-field of that war. We have come to dedicate a portion of that field, as a
final resting place for those who here gave their lives that that nation might
live. It is altogether fitting and proper that we should do this.

But, in a larger sense, we can not dedicate -- we can not consecrate -- we can
not hallow -- this ground. The brave men, living and dead, who struggled here,
have consecrated it, far above our poor power to add or detract. The world will
little note, nor long remember what we say here, but it can never forget what
they did here. It is for us the living, rather, to be dedicated here to the
unfinished work which they who fought here have thus far so nobly advanced. It
is rather for us to be here dedicated to the great task remaining before us --
that from these honored dead we take increased devotion to that cause for which
they gave the last full measure of devotion -- that we here highly resolve that
these dead shall not have died in vain -- that this nation, under God, shall
have a new birth of freedom -- and that government of the people, by the people,
for the people, shall not perish from the earth.

Abraham Lincoln
November 19, 1863
"#;
  let actual = correct_string(&String::from(input),&dict,&mut wc);
  let expect = r#"
Four score and seven years ago our fathers brought forth on this continent, a
new nation, !WRONG! in Liberty, and dedicated to the proposition that all men
are created equal.

Now we are engaged in a great civil war, testing whether that nation, or any
nation so !WRONG! and so dedicated, can long !WRONG!. We are met on a great
battle-field of that war. We have come to !WRONG! a portion of that field, as a
final !WRONG! place for those who here gave their lives that that nation might
live. It is !WRONG! fitting and proper that we should do this.

But, in a larger sense, we can not !WRONG! -- we can not !WRONG! -- we can
not !WRONG! -- this ground. The brave men, living and dead, who !WRONG! here,
have !WRONG! it, far above our poor power to add or !WRONG!. The world will
little note, nor long remember what we say here, but it can never forget what
they did here. It is for us the living, rather, to be dedicated here to the
!WRONG! work which they who fought here have thus far so !WRONG! advanced. It
is rather for us to be here dedicated to the great task remaining before us --
that from these !WRONG! dead we take increased !WRONG! to that cause for which
they gave the last full measure of !WRONG! -- that we here highly resolve that
these dead shall not have died in !WRONG! -- that this nation, under God, shall
have a new birth of freedom -- and that government of the people, by the people,
for the people, shall not !WRONG! from the earth.

Abraham Lincoln
November 19, 1863
"#;
  println!("expect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_correct_string5() {
  // checks combination of load_string_upper() with mark_corrected()
  // This test uses the full english dictionary and should find NO
  // corrections in the text.

  let fname = testdata("dict-full-english.txt");
  println!("fname: {}",fname);
  let dict = load_string_upper(&fname);
  let mut wc = Wrong{};
  let input  = r#"
Four score and seven years ago our fathers brought forth on this continent, a
new nation, conceived in Liberty, and dedicated to the proposition that all men
are created equal.

Now we are engaged in a great civil war, testing whether that nation, or any
nation so conceived and so dedicated, can long endure. We are met on a great
battle-field of that war. We have come to dedicate a portion of that field, as a
final resting place for those who here gave their lives that that nation might
live. It is altogether fitting and proper that we should do this.

But, in a larger sense, we can not dedicate -- we can not consecrate -- we can
not hallow -- this ground. The brave men, living and dead, who struggled here,
have consecrated it, far above our poor power to add or detract. The world will
little note, nor long remember what we say here, but it can never forget what
they did here. It is for us the living, rather, to be dedicated here to the
unfinished work which they who fought here have thus far so nobly advanced. It
is rather for us to be here dedicated to the great task remaining before us --
that from these honored dead we take increased devotion to that cause for which
they gave the last full measure of devotion -- that we here highly resolve that
these dead shall not have died in vain -- that this nation, under God, shall
have a new birth of freedom -- and that government of the people, by the people,
for the people, shall not perish from the earth.

Abraham Lincoln
November 19, 1863
"#;
  let actual = correct_string(&String::from(input),&dict,&mut wc);
  let expect = input;           // no corrections, all words in dictionary
  println!("expect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}


#[test]
fn test_mark_corrector_basics() {
  // ensures new() function is present along with correct_word()
  let mut mc = MarkCorrector::new(">>","<<");
  let actual = mc.correct_word("apple");
  let expect = ">>APPLE<<";
  println!("expect: {expect}\nactual: {actual}\n");
  assert!(expect ==  actual);

  let mut mc = MarkCorrector::new("attn:",":sp?");
  let actual = mc.correct_word("qwertyuiop");
  let expect = "attn:QWERTYUIOP:sp?";
  println!("expect: {expect}\nactual: {actual}\n");
  assert!(expect ==  actual);
}

#[test]
fn test_mark_corrector_correct_string1() {
  // uses correct_string() with a MarkCorrector
  let dict = v2s(vec!["APPLE","BANANA","ONION"]);              
  let mut mc = MarkCorrector::new("#(",")#");
  let input = "grape  123  apple --  onion";
  let actual = correct_string(&String::from(input),&dict,&mut mc);
  let expect ="#(GRAPE)#  123  apple --  onion";
  println!("expect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_mark_corrector_correct_string2() {
  // check specifically for non-whitespace separation such as commas,
  // semicolons, and numbers
  let dict = v2s(vec!["APPLE","BANANA","ONION"]);              
  let mut mc = MarkCorrector::new("<<",">>");
  let input  = "grape,pear,apple;;carrot[BANANA]onion1pear234grape";
  let actual = correct_string(&String::from(input),&dict,&mut mc);
  let expect = "<<GRAPE>>,<<PEAR>>,apple;;<<CARROT>>[BANANA]onion1<<PEAR>>234<<GRAPE>>";
  println!("expect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_mark_corrector_correct_string3() {
  // checks combination of load_string_upper() with correct_string()
  let fname = testdata("dict-google-100.txt");
  println!("fname: {}",fname);
  let dict = load_string_upper(&fname);
  let mut mc = MarkCorrector::new("attn:",":sp?");
  let input  = "\nAll work and no play makes Jack a dull boy...\nAll work and no...";
  let actual = correct_string(&String::from(input),&dict,&mut mc);
  let expect = "\nAll attn:WORK:sp? and no attn:PLAY:sp? attn:MAKES:sp? attn:JACK:sp? a attn:DULL:sp? attn:BOY:sp?...\nAll attn:WORK:sp? and no...";
  println!("expect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_auto_corrector_closest_word1() {
  let dict = v2s(vec!["ALL","BASE","ARE","YOUR","US"]);  // dict must be upcased before new()
  let ac = AutoCorrector::new(&dict,false);  

  let word = "bass";
  let (aword,adist) = ac.closest_word(word);
  let (eword,edist) = ("BASE",1);
  println!("word: {word}\nexpect: ({eword},{edist})\nactual: ({aword},{adist})");
  assert!(aword == eword && adist == edist);

  let word = "belong";
  let (aword,adist) = ac.closest_word(word);
  let (eword,edist) = ("ALL",5);
  println!("word: {word}\nexpect: ({eword},{edist})\nactual: ({aword},{adist})");
  assert!(aword == eword && adist == edist);
}

use edit_distance::edit_distance;


#[test]
fn test_auto_corrector_closest_word2() {
  let dict = v2s(vec!["A","B","C"]);   // dict must be upcased before new()
  let ac = AutoCorrector::new(&dict,false);
  
  let word = "b";
  let (aword,adist) = ac.closest_word(word);
  let (eword,edist) = ("B",0);
  println!("word: {word}\nexpect: ({eword},{edist})\nactual: ({aword},{adist})");
  assert!(aword == eword && adist == edist);

  let word = "cc";
  let (aword,adist) = ac.closest_word(word);
  let (eword,edist) = ("C",1);
  println!("word: {word}\nexpect: ({eword},{edist})\nactual: ({aword},{adist})");
  assert!(aword == eword && adist == edist);

  let word = "zzz";
  let (aword,adist) = ac.closest_word(word);
  let (eword,edist) = ("A",3);
  println!("word: {word}\nexpect: ({eword},{edist})\nactual: ({aword},{adist})");
  assert!(aword == eword && adist == edist);
}

#[test]
fn test_auto_corrector_closest_word3() {
  // Uses indicated dictionary file. To comply, must check words in
  // the order they appear in the dictionary and must use
  // edit_distance() as some of the closest words require its
  // calculation to find.
  let fname = testdata("dict-google-100.txt");
  println!("fname: {}",fname);
  let dict = load_string_upper(&fname);
  let ac = AutoCorrector::new(&dict,true);
  
  let word = "help";
  let (aword,adist) = ac.closest_word(word);
  let (eword,edist) = ("HELP",0);
  println!("word: {word}\nexpect: ({eword},{edist})\nactual: ({aword},{adist})");
  assert!(aword == eword && adist == edist);

  let word = "helped";
  let (aword,adist) = ac.closest_word(word);
  let (eword,edist) = ("HELP",2);
  println!("word: {word}\nexpect: ({eword},{edist})\nactual: ({aword},{adist})");
  assert!(aword == eword && adist == edist);

  let word = "hour";
  let (aword,adist) = ac.closest_word(word);
  let (eword,edist) = ("YOUR",1);
  println!("word: {word}\nexpect: ({eword},{edist})\nactual: ({aword},{adist})");
  assert!(aword == eword && adist == edist);

  let word = "Sandwhich";
  let (aword,adist) = ac.closest_word(word);
  let (eword,edist) = ("WHICH",4);
  println!("word: {word}\nexpect: ({eword},{edist})\nactual: ({aword},{adist})");
  assert!(aword == eword && adist == edist);

  let word = "formation";
  let (aword,adist) = ac.closest_word(word);
  let (eword,edist) = ("INFORMATION",2);
  println!("word: {word}\nexpect: ({eword},{edist})\nactual: ({aword},{adist})");
  assert!(aword == eword && adist == edist);

  let word = "infermtion";
  let (aword,adist) = ac.closest_word(word);
  let (eword,edist) = ("INFORMATION",2);
  println!("word: {word}\nexpect: ({eword},{edist})\nactual: ({aword},{adist})");
  assert!(aword == eword && adist == edist);
}


#[test]
fn test_ac_correct_word_show_sub_false1() {
  // checks correct_word() returns the closest word only. 
  let dict = v2s(vec!["ALL","BASE","ARE","YOUR","US"]);
  let mut ac = AutoCorrector::new(&dict,false); // false: don't show verbose corrections

  let word = "bass";
  let actual = ac.correct_word(word);
  let expect = "BASE";
  println!("word: {word}\nexpect: {expect}\nactual: {actual}");
  assert!(expect == actual);

  let word = "US";
  let actual = ac.correct_word(word);
  let expect = "US";
  println!("word: {word}\nexpect: {expect}\nactual: {actual}");
  assert!(expect == actual);

  let word = "belong";
  let actual = ac.correct_word(word);
  let expect = "ALL";
  println!("word: {word}\nexpect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_ac_correct_word_show_sub_false2() {
  // checks correct_word() returns the closest word only. 
  let fname = testdata("dict-google-100.txt");
  println!("fname: {}",fname);
  let dict = load_string_upper(&fname);
  let mut ac = AutoCorrector::new(&dict,false); // false: don't show verbose corrections

  let word = "hour";
  let actual = ac.correct_word(word);
  let expect = "YOUR";
  println!("word: {word}\nexpect: {expect}\nactual: {actual}");
  assert!(expect == actual);

  let word = "Sandwhich";
  let actual = ac.correct_word(word);
  let expect = "WHICH";
  println!("word: {word}\nexpect: {expect}\nactual: {actual}");
  assert!(expect == actual);

  let word = "infermtion";
  let actual = ac.correct_word(word);
  let expect = "INFORMATION";
  println!("word: {word}\nexpect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_ac_correct_word_show_sub_true1() {
  // checks correct_word() returns the closest word with additional
  // information on the substitution
  let dict = v2s(vec!["ALL","BASE","ARE","YOUR","US"]);
  let mut ac = AutoCorrector::new(&dict,true); // true: show verbose corrections

  let word = "bass";
  let actual = ac.correct_word(word);
  let expect = "(bass:BASE:1)";
  println!("word: {word}\nexpect: {expect}\nactual: {actual}");
  assert!(expect == actual);

  let word = "US";
  let actual = ac.correct_word(word);
  let expect = "(US:US:0)";
  println!("word: {word}\nexpect: {expect}\nactual: {actual}");
  assert!(expect == actual);

  let word = "belong";
  let actual = ac.correct_word(word);
  let expect = "(belong:ALL:5)";
  println!("word: {word}\nexpect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_ac_correct_word_show_sub_true2() {
  // checks correct_word() returns the closest word only. 
  let fname = testdata("dict-google-100.txt");
  println!("fname: {}",fname);
  let dict = load_string_upper(&fname);
  let mut ac = AutoCorrector::new(&dict,true); // true: show verbose corrections

  let word = "hour";
  let actual = ac.correct_word(word);
  let expect = "(hour:YOUR:1)";
  println!("word: {word}\nexpect: {expect}\nactual: {actual}");
  assert!(expect == actual);

  let word = "Sandwhich";
  let actual = ac.correct_word(word);
  let expect = "(Sandwhich:WHICH:4)";
  println!("word: {word}\nexpect: {expect}\nactual: {actual}");
  assert!(expect == actual);

  let word = "infermtion";
  let actual = ac.correct_word(word);
  let expect = "(infermtion:INFORMATION:2)";
  println!("word: {word}\nexpect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}


#[test]
fn test_ac_false_correct_string1() {
  // uses correct_string() with an AutoCorrector
  let dict = v2s(vec!["APPLE","BANANA","ONION"]);              
  let mut ac = AutoCorrector::new(&dict, false);
  let input = "grape  123  apple --  onion";
  let actual = correct_string(&String::from(input),&dict,&mut ac);
  let expect = "APPLE  123  apple --  onion";
  println!("input:  {input}\nexpect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_ac_false_correct_string2() {
  // uses correct_string() with an AutoCorrector
  let dict = v2s(vec!["APPLE","BANANA","ONION"]);              
  let mut ac = AutoCorrector::new(&dict, false);
  let input  = "grape pear apple carrot bannana onon pear grape";
  let actual = correct_string(&String::from(input),&dict,&mut ac);
  let expect = "APPLE APPLE apple APPLE BANANA ONION APPLE APPLE";
  println!("input:  {input}\nexpect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_ac_true_correct_string1() {
  // uses correct_string() with an AutoCorrector
  let dict = v2s(vec!["APPLE","BANANA","ONION"]);              
  let mut ac = AutoCorrector::new(&dict, true); // verbose corrections
  let input = "grape  123  apple --  onion";
  let actual = correct_string(&String::from(input),&dict,&mut ac);
  let expect = "(grape:APPLE:4)  123  apple --  onion";
  println!("input:  {input}\nexpect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_ac_true_correct_string2() {
  // uses correct_string() with an AutoCorrector
  let dict = v2s(vec!["APPLE","BANANA","ONION"]);              
  let mut ac = AutoCorrector::new(&dict, true); // verbose corrections
  let input  = "grape pear apple carrot bannana onon pear grape";
  let actual = correct_string(&String::from(input),&dict,&mut ac);
  let expect = "(grape:APPLE:4) (pear:APPLE:4) apple (carrot:APPLE:5) (bannana:BANANA:1) (onon:ONION:1) (pear:APPLE:4) (grape:APPLE:4)";
  println!("input:  {input}\nexpect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_ac_false_correct_string3() {
  // combines correct_string() with dictionary loading
  let fname = testdata("dict-google-10000.txt");
  println!("fname: {}",fname);
  let dict = load_string_upper(&fname);
  let mut ac = AutoCorrector::new(&dict, false); // only correct words
  let input  = r#"
Four score and seven years ago our fathers brought forth on this continent, a
new nation, conceived in Liberty, and dedicated to the proposition that all men
are created equal.

Now we are engaged in a great civil war, testing whether that nation, or any
nation so conceived and so dedicated, can long endure. We are met on a great
battle-field of that war. We have come to dedicate a portion of that field, as a
final resting place for those who here gave their lives that that nation might
live. It is altogether fitting and proper that we should do this.

But, in a larger sense, we can not dedicate -- we can not consecrate -- we can
not hallow -- this ground. The brave men, living and dead, who struggled here,
have consecrated it, far above our poor power to add or detract. The world will
little note, nor long remember what we say here, but it can never forget what
they did here. It is for us the living, rather, to be dedicated here to the
unfinished work which they who fought here have thus far so nobly advanced. It
is rather for us to be here dedicated to the great task remaining before us --
that from these honored dead we take increased devotion to that cause for which
they gave the last full measure of devotion -- that we here highly resolve that
these dead shall not have died in vain -- that this nation, under God, shall
have a new birth of freedom -- and that government of the people, by the people,
for the people, shall not perish from the earth.

Abraham Lincoln
November 19, 1863
"#;
  let actual = correct_string(&String::from(input),&dict,&mut ac);
  let expect = r#"
Four score and seven years ago our fathers brought forth on this continent, a
new nation, CONCERNED in Liberty, and dedicated to the proposition that all men
are created equal.

Now we are engaged in a great civil war, testing whether that nation, or any
nation so CONCERNED and so dedicated, can long ENSURE. We are met on a great
battle-field of that war. We have come to DEDICATED a portion of that field, as a
final TESTING place for those who here gave their lives that that nation might
live. It is TOGETHER fitting and proper that we should do this.

But, in a larger sense, we can not DEDICATED -- we can not CONCRETE -- we can
not ALLOW -- this ground. The brave men, living and dead, who STRUGGLE here,
have CONNECTED it, far above our poor power to add or DETROIT. The world will
little note, nor long remember what we say here, but it can never forget what
they did here. It is for us the living, rather, to be dedicated here to the
FINISHED work which they who fought here have thus far so NOBLE advanced. It
is rather for us to be here dedicated to the great task remaining before us --
that from these HUNDRED dead we take increased DEVIATION to that cause for which
they gave the last full measure of DEVIATION -- that we here highly resolve that
these dead shall not have died in MAIN -- that this nation, under God, shall
have a new birth of freedom -- and that government of the people, by the people,
for the people, shall not PARISH from the earth.

Abraham Lincoln
November 19, 1863
"#;
  println!("expect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}

#[test]
fn test_ac_true_correct_string3() {
  // combines correct_string() with dictionary loading
  let fname = testdata("dict-google-10000.txt");
  println!("fname: {}",fname);
  let dict = load_string_upper(&fname);
  let mut ac = AutoCorrector::new(&dict, true); // verbose corrections
  let input  = r#"
Four score and seven years ago our fathers brought forth on this continent, a
new nation, conceived in Liberty, and dedicated to the proposition that all men
are created equal.

Now we are engaged in a great civil war, testing whether that nation, or any
nation so conceived and so dedicated, can long endure. We are met on a great
battle-field of that war. We have come to dedicate a portion of that field, as a
final resting place for those who here gave their lives that that nation might
live. It is altogether fitting and proper that we should do this.

But, in a larger sense, we can not dedicate -- we can not consecrate -- we can
not hallow -- this ground. The brave men, living and dead, who struggled here,
have consecrated it, far above our poor power to add or detract. The world will
little note, nor long remember what we say here, but it can never forget what
they did here. It is for us the living, rather, to be dedicated here to the
unfinished work which they who fought here have thus far so nobly advanced. It
is rather for us to be here dedicated to the great task remaining before us --
that from these honored dead we take increased devotion to that cause for which
they gave the last full measure of devotion -- that we here highly resolve that
these dead shall not have died in vain -- that this nation, under God, shall
have a new birth of freedom -- and that government of the people, by the people,
for the people, shall not perish from the earth.

Abraham Lincoln
November 19, 1863
"#;
  let actual = correct_string(&String::from(input),&dict,&mut ac);
  let expect = r#"
Four score and seven years ago our fathers brought forth on this continent, a
new nation, (conceived:CONCERNED:2) in Liberty, and dedicated to the proposition that all men
are created equal.

Now we are engaged in a great civil war, testing whether that nation, or any
nation so (conceived:CONCERNED:2) and so dedicated, can long (endure:ENSURE:1). We are met on a great
battle-field of that war. We have come to (dedicate:DEDICATED:1) a portion of that field, as a
final (resting:TESTING:1) place for those who here gave their lives that that nation might
live. It is (altogether:TOGETHER:2) fitting and proper that we should do this.

But, in a larger sense, we can not (dedicate:DEDICATED:1) -- we can not (consecrate:CONCRETE:3) -- we can
not (hallow:ALLOW:1) -- this ground. The brave men, living and dead, who (struggled:STRUGGLE:1) here,
have (consecrated:CONNECTED:3) it, far above our poor power to add or (detract:DETROIT:2). The world will
little note, nor long remember what we say here, but it can never forget what
they did here. It is for us the living, rather, to be dedicated here to the
(unfinished:FINISHED:2) work which they who fought here have thus far so (nobly:NOBLE:1) advanced. It
is rather for us to be here dedicated to the great task remaining before us --
that from these (honored:HUNDRED:2) dead we take increased (devotion:DEVIATION:2) to that cause for which
they gave the last full measure of (devotion:DEVIATION:2) -- that we here highly resolve that
these dead shall not have died in (vain:MAIN:1) -- that this nation, under God, shall
have a new birth of freedom -- and that government of the people, by the people,
for the people, shall not (perish:PARISH:1) from the earth.

Abraham Lincoln
November 19, 1863
"#;
  println!("expect: {expect}\nactual: {actual}");
  assert!(expect == actual);
}
