#![allow(unused_variables)] // allow unused variables
#![allow(dead_code)] // allow unused functions

pub fn run(){
    println!("Topic: Numbers");
    // int1();
    // int2();
    // int3();
    // int4();
    // int5();
    // int6();
    // fp7();
    // fp8();
    // r9();
    r10();
}


// ============== Integer ============== //
fn int1() {
    let x: i32 = 5;
    let mut _y: i32 = 5; // underscore to mark unused variable

    _y = x;
    
    let _z: i32 = 10; 

    println!("Success! 1");
}

fn int2(){
    
    let v: u16 = 38_u8 as u16;
    println!("Success! 2");
}

fn int3(){


    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x));

    println!("Success! 3");


    // Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }
}

fn int4(){
    assert_eq!(i8::MAX, 127); // max of i8 (integer 8 bit) is 127
    assert_eq!(u8::MAX, 255); // max of u8 (unsigned integer 8 bit) is 255

    println!("Success! 4");
}

fn int5(){

   let v1 = 251_i16 + 8;
   let v2 = i16::checked_add(251, 8).unwrap();
   println!("{},{}",v1,v2);

}

fn int6(){

    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; // 1024 + 255 + 63 + 255 = 1597
    assert!(v == 1597); // change 1597 to 1597

    println!("Success! 6");
}


// ============== Floating-Point ============== //

fn fp7(){
    

    let x: f64 = 1_000.000_1; // make f64
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of(&x), "f64".to_string()); // put f64
    println!("Success! 7");

    fn type_of<T>(_: &T) -> String {
        format!("{}", std::any::type_name::<T>())
    }
}

fn fp8(){
    
    assert!(0.1_f32+0.2_f32==0.3_f32); // put _f32 as suffix for typecasting
    println!("Success! 8");

}


// ============== Range (for loops) ============== //
fn r9(){

    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}",c as i32);
    }
}

fn r10(){

    use std::ops::{Range, RangeInclusive};

    assert_eq!((1..5), Range{ start: 1, end: 5 }); // not inclusive, will end in 4 (5 not included)
    assert_eq!((1..=5), RangeInclusive::new(1, 5)); // inclusive, will end in 5 (5 included)

    println!("Success! 10");

}
