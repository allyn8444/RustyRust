#![allow(unused_variables)] // allow unused variables
#![allow(dead_code)] // allow unused functions

pub fn run_string() {
    println!("Compound Types: String - 6.1 Topic");
    // println!("This is submodule - string");

    // =============== Exercises =============== //

    // s1();
    // s2();
    // s3();
    // s4();
    // s5();
    // s6();
    // s7();
    // s8();
    // s9();
    // s10();
}



// Fix error without adding new line
fn s1() {
    let s: &str = "hello, world"; // Solution: change str to &str

    println!("Success! - 1");
} 


// Fix the error with at least two solutions
fn s2() {
    let s: &str = "hello, world".into(); // Solution: make into &str
    greetings(s); // put semicolon at the end of the line

    fn greetings(s: &str) {
        println!("{}",s)
    }
}


// Fill the blank
fn s3() {
    let mut s = String::from(""); // Solution: make it String type but empty
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success! - 3");
}


// Fix all errors without adding newline
fn s4() {
    let mut s = String::from("hello"); // make variable s mutable
    s.push_str(","); // change push to push_str
    s.push_str(" world"); // change push to push_str
    s += "!"; // remove .to_string()

    println!("{}", s);
}


// Fill the blank
fn s5() {
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats");
    // Solution: use replace method to replace "dogs" with "cats"

    assert_eq!(s1, "I like cats");

    println!("Success! - 5");
}



// Fix errors without removing any line
fn s6() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2; 
    // Solution: clone s1 to avoid moving it (because it is used later)
    // Solution: use &s2 to get reference of s2

    assert_eq!(s3, "hello,world!");
    println!("{}", s1);

    println!("Success! - 6");
}



// Fix error with at least two solutions
fn s7() {
    let s: &str = "hello, world"; //Solution: either change to &str or String type
    greetings(s);

    fn greetings(s:  &str) { //Solution: either change to &str or String type
        println!("{}", s)
    }
}



// Use two approaches to fix the error and without adding a new line
fn s8() {
    // let s = "hello, world".to_string(); // This makes it a String type
    // let s1: String = s; // Solution: change &str to String type

    let s : &str= "hello, world"; //Solution 2: remove .to_string() to make it &str
    let s1: &str = s; 

    println!("Success! - 8");
}



// AN EXAMPLE, NOT AN EXERCISE
fn s9() {
    // You can use escapes to write bytes by their hexadecimal values
    // Fill the blank below to show "I'm writing Rust"
    let byte_escape = "I'm writing Ru\x73t!"; // \x73 is a byte escape sequence representing the ASCII character 's', 
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...Or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!("Unicode character {} (U+211D) is called {}",
                unicode_codepoint, character_name );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}




/* Fill in the blank and fix the errors */
fn s10() {
    let raw_str = "Escapes don't work here: \x3F \u{211D}"; // remove the r 
    // Modify above line to make it work
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // If you need quotes in a raw string, add a pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // If you need "# in your string, just use more #s in the delimiter.
    // You can use up to 65535 #s.
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // Fill the blank
    let long_delimiter = "Hello, \"##\""; // copy the delimiter above in assert_eq
    assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Success!");
}