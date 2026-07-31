// Functions
//any function / variables should be written in snake case
// snake case: hello_world
fn main() {
    hello();
}

// Hoisting supported by Rust but many programming languages don't supported it. 
// Hoisting means you can write functions above or bellow of the main function.
fn hello() {
    println!("Hello, Rust!");
}

fn tell_height(height: i32) {
    println!("My height is: {}", height);
}