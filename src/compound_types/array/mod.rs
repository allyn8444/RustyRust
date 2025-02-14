#![allow(unused_variables)] // allow unused variables
#![allow(dead_code)] // allow unused functions


pub fn run_array(){
     println!("Compound Types: Array - 6.2 Topic");
    // println!("This is submodule - array");

    // =============== Exercises =============== //

    // ar1();
    // ar2();
    // ar3();
    // ar4();
    // ar5();
    ar6();
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
    let arr: [char; 3] = ['a', 'b', 'c'];
    // Solution: Data type -> char
    
    // Fill the blank
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 12);
    // Solution: every char is 4 bytes and we have 3 chars in the array
    // so 4bytes * 3 = 12bytes
    // answer is 12

    println!("Success! - 2");
}



fn ar3() {
    // Fill the blank
    let list: [i32; 100] = [1; 100];
    // Solution: [value; length]
    // 100 items but all are 1

    assert!(list[1] == 1); // test here
    assert!(list.len() == 100);

    println!("Success! - 3");
}


fn ar4() {
    // Fix the error
    let _arr:[i32; 3] = [1, 2, 3]; // Solution: make the char 3 as int 3

    println!("Success! - 4");
}



fn ar5() {
    let arr = ['a', 'b', 'c'];
    
    let ele = arr[0]; // Only modify this line to make the code work!
    // Solution: change the index to 0

    assert!(ele == 'a');

    println!("Success! - 5");
}




// Fix the error
fn ar6() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    
    // `Get` returns an Option<T>, it's safe to use
    let name0 = names.get(0).unwrap();

    // But indexing is not safe
    let _name1 = &names[1]; // out of bounds

    println!("Success! - 6");
}




