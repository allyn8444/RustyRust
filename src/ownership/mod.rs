#![allow(unused_variables)] // allow unused variables
#![allow(dead_code)] // allow unused functions


pub fn run(){
    println!("Ownership 5.1 Topic");

    // own_exp1();
    // own_exp2();

    // own1();
    // own2();
    // own3();
    // own4();
    // own5();
    // own6();
    // own7();
    // partial_move_example()
    // own8();
    own9();
}


// For explanation purposes:
fn own_exp1(){

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


fn own_exp2(){

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



// ---------- Ownership ---------- //

fn own1() {
    // Use as many approaches as you can to make it work
    let x = String::from("Hello world");
    let y = x.clone(); // use .clone() method
    println!("{}, {}",x, y);
}


fn own2() {
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


fn own3() {
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
fn own4() {
    let s = String::from("Hello World");

    print_str(s.clone());  // make into s.clone()

    println!("{}", s);

    fn print_str(s: String) { // return the s value
        println!("{}",s);
    }
}



fn own5() {
    let x : (i32, i32, (), &str) = (1, 2, (), "hello"); // make sure to put data types of each values in the tuple
    // use &str, string literal, instead of String type
    // &str is immutable and String type is mutable
    let y : (i32, i32, (), &str) = x; // copy the data types from x and remove the .clone()
    println!("{:?}, {:?}", x, y);
}
/*
=== EXPLANATION ====

Why &str Works but Not String:

- &str is a reference to a statically allocated string, and it implements the Copy trait.
- Copying a &str simply involves duplicating the pointer and length, which is cheap.

- String is a heap-allocated, owned type, and moving it requires transferring ownership.
- Since Rust enforces strict ownership rules, this causes problems if you try to use the original variable (x) after the move.



Summary
- Use &str when you don’t need ownership of the string and can work with a reference.
- Use String when you need an owned, mutable string.
- If you want to copy a tuple containing a String, you must explicitly clone, .clone(), the String to avoid ownership issues.

*/



// ---------- Mutability ---------- //


fn own6() {

    // make the necessary variable mutable
    let s = String::from("Hello ");
    
    let mut s1 = s; // put mut keyword

    s1.push_str("World!"); // you can't do this to &str as they're immutable 

    println!("{}", s1);
}


fn own7() {
    let x = Box::new(5);       // Allocate an integer (5) on the heap and bind it to x
    // 5 is i32 type that is stored in Stack Memory (static)
    // when Box is used, 5 i32 will be moved into Heap Memory making it dynamic 

    let mut y = Box::new(1);   // Allocate a separate integer (1) on the heap and bind it to y
    
    *y = 4;                    // Dereference y and assign the value 4 to the heap location
    
    assert_eq!(*x, 5);         // Assert that x's value is still 5 (unchanged)
    
    println!("Success!");      // Print "Success!" if the program runs without issues
}



// ---------- Partial Move ---------- //

// very complex example for now
fn partial_move_example() {
    
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    // `name` is moved out of person, but `age` is referenced
    let Person { name, ref age } = person;

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial move occurs
    //println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);
}




fn own8() {
   let t = (String::from("hello"), String::from("world"));

   let _s = t.0; // assign the first value of tuple in _s
   // Now, the ownership of t.0 is assigned to _s

   // Modify this line only, don't use `_s`
   println!("{:?}", t.1); // we can still access t.1 since it is only t.0 ownership's that has been transferred 
}


fn own9() {
   let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (s1, s2) = t.clone();
    // clone t first using .clone() method so that you can access t later
    // and then destructure

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
    
}



