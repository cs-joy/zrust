// Shadowing
// We can declare a new variable with the same name as a previous variable. Rustaceans say that
// the first variable is shadowed by the second, which means that the second variable is what the
// compiler will see when you use thate name of variable....
// @ref https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing
// \
// Shadowing simply means you create a variable with the same name and you are shadowing the first one

// One more important point that shadowing is different from marking a variable as mutable (`mut`)
// How? so simply if you accidentally try to reassign to this variable without using the `let`
// keyword, you will have a compile time error. for example
// let x = 3;
// x = x+1; // it will generate an error as "cannot assign twice to immutable variable"

// and other difference between mute and shadowing is that because we are creating a new variable 

// fn main() {
//     let x = 6; // result x = 6
//     let x = x + 1; // result x = 7 // here `x+1` x is first variable which is shadowing
//     println!("The value of the second variable (x) is: {}", x);
//     // we can create many more variables
//     let x = x + 1; // result x = 8
//     // and so on
//     println!("The value of the third variable (x) is: {}", x);

//     // inner scope
//     {
//         let x = x * 2; // result x = 16
//         println!("The value of x in the inner scope is: {}", x);
//     }
//     println!("The value of the third variable (x) is: {}", x);
// }


// The other difference between `mut` and shadowing is that because we're
// effectively creating a new variable when we use the `let` keyword again, 
// we can change the type of the value but reuse the same name.
// For example,
// say our program asks a user to show how much spaces they want between 
// some text by inputting space characters, and then we want to store that 
// input as a numeber:
fn main() {
    let spaces = "    "; // string type
    let spaces = spaces.len(); // number type
    println!("spaces: {}", spaces);

    // if we try to use `mut` for this, it will generate a compile time error
    // let mut spaces_2 = "    ";
    // spaces_2 = spaces_2.len();
    // println!("spaces_2: {}", spaces_2);
}
