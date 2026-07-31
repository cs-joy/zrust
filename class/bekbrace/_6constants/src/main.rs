// Constants
// Constants that are values that are bound to a name and not allowed to change exactly like variable.
// But in fact there are few differences between constants and variables. That's why we are very serious about this terms.

// Granted both are immutable but the first difference between constants and variable 
// that in a constant you are no allowed to use the keyword `mut` with constants. So
// if you will do:

fn main() {
    println!("Constants");
    let mut x = 5;
    // const mut y = 10; // it will generate an error. because we must need to do type annotation when we will declare a constant variable
    // so
    // const mut y: i32 = 10; // it will generate an error too. because as we know, this can't be mutable
    println!("The value of x is: {}", x);
    // println!("The value of y is: {}", y);

    // so the based on the correct rules in Rust, we shold define like this
    const Y: i32 = 10;
    // const y: i32 = 10; // it will generate an error
    println!("The value of Y is: {}", Y);

    // print PI value
    println!("The value of PI is: {}", PI);

    println!("The value of 3 Hours in seconds is: {}", THREE_HOURS_IN_SECONDS);
}

// Also distinct feature of const is that you can declare a const global scope.
// Global Scope means outside of the main function.
// so,, this is where we can declare a constant with a type annotation
const PI: f64 = 3.141592653;

// another example: https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#variables-and-mutability
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;