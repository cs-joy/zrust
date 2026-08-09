// If Else [ If expression ] [ Else expression ]

// to keep disable the warning (from the compiler)
#![allow(warnings)]
fn main() {
    // let age: u16 = 18;
    // if age >= 18 {
    //     println!("You can drive a car!");
    // } else {
    //     println!("You can't drive a car!");
    // }

    // another example::

    // Multiple conditions with else if:
    // let number = 6;
    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2")
    // } else {
    //     print!("number is not divisible by 4, 3, or 2");
    // }

    // another example
    // Using if in a let statement
    let condition = true;
    let number = if condition {5} else {6};
    println!("number: {}", number);
    
}
