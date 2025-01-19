mod numbers;

fn main(){
    println!("Starting Rust...");
    println!("Ended -> 1:29:00 ");

    numbers::function();   
}


/*

fn main() {
    // Don't modify the following two lines!
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32{
    x + y
}



fn main() {
   print();
}

// Replace i32 with another type
fn print() -> () {
   println!("Success!");
}


// Solve it in two ways
// DON'T let `println!` work
fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    // Implement this function, don't modify the fn signatures
    panic!()
}




*/