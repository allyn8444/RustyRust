#![allow(unused_variables)] // allow unused variables
#![allow(dead_code)] // allow unused functions


pub fn run_slice(){
     println!("Compound Types: Slice - 6.3 Topic");
    // println!("This is submodule - array");

    // =============== Exercises =============== //


    // sl1();
    // sl2();
    // sl3();
    // sl4();
    // sl5();
    sl6();
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



fn sl4() {
    let s: String = String::from("hello");

    let slice1:&str = &s[0..2];
    // Fill the blank to make the code work, DON'T USE 0..2 again
    let slice2 = &s[..2];
    // Solution: &s[..2]
    //  we can omit the zero (it is understandable that it will start at index zero)

    assert_eq!(slice1, slice2);

    println!("Success! - 4");
}



fn sl5() {
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[..3]; // Solution: change 2 to 3
 
    assert!(slice == "你");

    println!("Success! - 5");
}


// Fix errors
fn sl6() {
    let mut s = String::from("hello world");

    // Here, &s is `&String` type, but `first_letter` needs a `&str` type.
    // It works because `&String` can be implicitly converted to `&str. If you want to know more, this is called `Deref coercion`. 
    let letter = first_letter(&s);

    //  s.clear(); // error! 
     // Solution: put this after the println
     // cant use before the println since we are using the value for the `letter`

    println!("the first letter is: {}", letter);
    
    s.clear();


    fn first_letter(s: &str) -> &str {
        &s[..1]
    }

}
