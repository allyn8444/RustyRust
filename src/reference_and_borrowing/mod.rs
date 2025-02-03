#![allow(unused_variables)] // allow unused variables
#![allow(dead_code)] // allow unused functions

pub fn run(){
    println!("Reference and Borrowing 5.2 Topic");

    // rb1();
    // rb2();
    // rb3();
    rb4();
}




fn rb1() {
   let x = 5;
   // Fill the blank
   let p = &x; // use ampersand (&) to get memory address of a value

   println!("the memory address of x is {:p}", p); // One possible output: 0x16fa3ac84
}




fn rb2() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y); // use asterisk (*) to dereference a value (the process of accessing or changing the value stored in a memory address that a pointer points to)

    println!("Success!");
}





fn rb3() {
    // Fix error
    let s = String::from("hello, "); // doesn't need to be mutable

    borrow_object(s);

    println!("Success!");
}

fn borrow_object(s: String) -> String{ // change &String to String
    // Put return value -> String
    // return s

    s
}




// Fix error
fn rb4() {
    let mut s = String::from("hello, ");

    // need to put mutable reference of s
    push_str(&mut s);

    println!("Success!");
}

fn push_str(s: &mut String) {
    s.push_str("world")
}








