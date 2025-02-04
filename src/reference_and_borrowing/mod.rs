#![allow(unused_variables)] // allow unused variables
#![allow(dead_code)] // allow unused functions

pub fn run(){
    println!("Reference and Borrowing 5.2 Topic");

    // rb1();
    // rb2();
    // rb3();
    // rb4();
    // rb5();
    // rb6();
    // rb7();
    // rb8();
    // rb9();
    // rb10();
    rb11();
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

    println!("Success! - 2");
}





fn rb3() {
    // Fix error
    let s = String::from("hello, "); // doesn't need to be mutable

    borrow_object(s);

    println!("Success! - 3");


    fn borrow_object(s: String) -> String{ // change &String to String
    // Put return value -> String
    // return s
        s
    }

}





// Fix error
fn rb4() {
    let mut s = String::from("hello, ");

    // need to put mutable reference of s
    push_str(&mut s);

    println!("Success! - 4");


    fn push_str(s: &mut String) {
        s.push_str("world")
    }

}




fn rb5() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = &mut s; // Solution: use &mut to get mutable reference of s
    
    p.push_str("world");

    println!("Success! - 5");
}






fn rb6() {
    let c = '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let ref r2 = c; // Solution: use ref to get reference of c

    assert_eq!(*r1, *r2);
    
    // Check the equality of the two address strings
    assert_eq!(get_addr(r1),get_addr(r2));

    println!("Success! - 6");


    // Get memory address string
    fn get_addr(r: &char) -> String {
        format!("{:p}", r)
    }
}






// Remove something to make it work
// Don't remove a whole line !
fn rb7() {
    let s = String::from("hello"); 
    // Solution: remove mut keyword. use immutable reference instead of mutable reference
    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    println!("Success! - 7");
}



fn rb8() {
    // Fix error by modifying this line
    let mut s = String::from("hello, "); // Solution: make s mutable

    borrow_object(&mut s);

    println!("Success! - 8");

    fn borrow_object(s: &mut String) {}
}


// This code has no errors!
fn rb9() {
    let mut s = String::from("hello, ");

    borrow_object(&s);
    
    s.push_str("world");

    println!("Success! - 9");

    
    fn borrow_object(s: &String) {}
}




// Comment one line to make it work
fn rb10() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");
    
    // println!("{}",r1); // Solution: comment this line
    // Explained:
    // Rust's compiler allows the second mutable borrow if the first one is no longer used
    println!("Success! - 10");
}





fn rb11() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same 
    // Solution:
    println!("{}",r1); 
}
