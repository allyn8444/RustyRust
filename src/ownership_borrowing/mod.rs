#![allow(unused_variables)] // allow unused variables
#![allow(dead_code)] // allow unused functions


pub fn run(){
    println!("Ownership and Borrowing");

    // ob_exp1();
    // ob_exp2();

    // ob1();
    // ob2();
    // ob3();
    ob4();
}


// For explanation purposes:
fn ob_exp1(){

    let s = String::from("hello"); // s comes into scope
    takes_ownership(s);       // s value MOVES into the function
    // println!("{}", s); // returns an error. Cuz s value is now in the function (moved)

    let x = 5;                       // x comes into scope
    makes_copy(x);          // x value COPIED into the function
    println!("{}", x);                   // x value is still accessible cuz it was not moved but rather just copied

    
    fn takes_ownership(some_string: String){
        println!("String moved -> {}", some_string);
    }

    fn makes_copy(some_integer: i32){
        println!("Copy of integer -> {}", some_integer);
    }
}


fn ob_exp2(){

    let s1 = gives_ownership(); // "yours" is now inside s1

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back( s2); // takes_and_gives_back gets the value from s2
                                                         // and then gives back the value to s3


    println!("S1 value -> {}",s1);
    // println!("{}",s2); // will return an error cuz s2 value was moved to s3
    println!("S3 value -> {}",s3);

    
    fn gives_ownership() -> String {
        let some_string = String::from("yours");
        // return
        some_string
    }

    fn takes_and_gives_back(a_string: String) -> String {

        // returning the string makes the difference
        a_string
    }

}


// Exercises:

fn ob1() {
    // Use as many approaches as you can to make it work
    let x = String::from("Hello world");
    let y = x.clone(); // use .clone() method
    println!("{}, {}",x, y);
}


fn ob2() {
    // Don't modify code here!
    let s1 = String::from("Hello world");
    let s2 = take_ownership(s1);

    println!("{}", s2);

    // Only modify the code below!
    fn take_ownership(s: String) -> String {
        println!("{}", s);
        
        // return 
        s
    }
}


fn ob3() {
    let s:String = give_ownership();
    println!("{}", s);


    // Only modify the code below!
fn give_ownership() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.as_bytes(); // turn into_bytes to as_bytes()
    s
}
}


// Fix the error without removing any code
fn ob4() {
    let s = String::from("Hello World");

    print_str(s.clone());  // make into s.clone()

    println!("{}", s);

    fn print_str(s: String) { // return the s value
        println!("{}",s);
    }
}


// TODO: not yet finished