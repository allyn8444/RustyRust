#![allow(unused_imports)]

// Declare submodule
pub mod string;
pub mod array;
pub mod slice;

// Re-export for convenient use
pub use string::run_string; // Bring into scope of main.rs
pub use array::run_array;
pub use slice::run_slice;