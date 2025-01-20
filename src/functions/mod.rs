#![allow(unused_variables)] // allow unused variables
#![allow(dead_code)] // allow unused functions


pub fn run(){
    println!("Topic: Functions");
    // f1();
    // f2();
    // f3();
    // f4();
    f5();

}


fn f1() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success! 1");


    
    fn sum(x: i32, y: i32) -> i32{
        x + y
    }
}


fn f2() {
   print();

   // Replace i32 with another type
    fn print() -> () { // this, (), is a unit type. Analogous to a void (as it doesn't return any type of datatype)
    println!("Success! 2");
    }
}


fn f3() {
    never_return();

    // DON'T let `println!` work
    // println!("Failed!"); // uncomment later


    fn never_return() -> ! {
    panic!() // panic automatically ends the runtime and return an error (unreachable statement)
    }

}


/*
Diverging functions are often used in code where you want to ensure the program
doesn't proceed after a certain point.

This can make your logic more explicit and eliminate invalid states.

*/


fn f4(){
    

    println!("Success! 4");


    fn get_option(tp: u8) -> Option<i32> {
        // switch block in Rust
        match tp {
            1 => {
                // TODO
            }
            _ => {
                // TODO
            }
        };
        
        // Rather than returning a None, we use a diverging function instead
        never_return_fn()
    }

    // IMPLEMENT this function in THREE ways
    fn never_return_fn() -> ! {
        panic!() // can also use todo!() or unimplemented!()
         
    }
}


fn f5(){

    // FILL in the blank
    let b = false; // turn into false

    let _v = match b {
        true => 1,
        // Diverging functions can also be used in match expression to replace a value of any value
        false => {
            println!("Success! 5");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!");

}