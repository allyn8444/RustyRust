#![allow(unused_variables)] // allow unused variables
#![allow(dead_code)] // allow unused functions


pub fn run_array(){
     println!("Compound Types: Array - 6.2 Topic");
    // println!("This is submodule - array");

    // =============== Exercises =============== //

    // ar1();
    ar2();
}


fn ar1() {
    // Fill the blank with proper array type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Solution: [values_type; length]

    // Modify the code below to make it work
    assert!(arr.len() == 5); // turn 4 into 5

    println!("Success! - 1");
}



// WATCH VIDEO
fn ar2() {
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];
    
    // Fill the blank
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == __);

    println!("Success!");
}