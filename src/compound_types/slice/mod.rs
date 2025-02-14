#![allow(unused_variables)] // allow unused variables
#![allow(dead_code)] // allow unused functions


pub fn run_slice(){
     println!("Compound Types: Slice - 6.3 Topic");
    // println!("This is submodule - array");

    // =============== Exercises =============== //


    // sl1();
    // sl2();
    sl3();
}

/*
    Slices are similar to arrays, but their length is
    not known at compile time, so you can't use slice directly.


    Slice Nature:
    - Slices are always References `&`
    - They don't own the data they're referencing
    - They're a view into a contiguous sequence of elements

*/



// Fix the errors, DON'T add new lines!
fn sl1() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2]; // s1 borrows a view of arr's elements

    let s2: &str = "hello, world" as &str;

    println!("Success! - 1");
}


fn sl2() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];
    
    // Modify '8' to make it work
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&slice) == 16);
    // Solution: 16
    // because slice calculates the pointer size + length size
    // pointer(8) + length(8) = 16

    println!("Success! - 2 ");
}




fn sl3() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    let slice: &[i32] = &arr[1..4];
    // Solution:
    // explicitly state i32 reference array type `&[i32]`
    // and slice the array from index 1 to 4

    assert_eq!(slice, &[2, 3, 4]);

    println!("Success! - 3");
}