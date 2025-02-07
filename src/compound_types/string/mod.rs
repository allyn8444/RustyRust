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
    s6();
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

