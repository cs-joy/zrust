// Variables and Mutability
// 
// In Rust, Variables are immutable by default. This is something you need to know.
// Immutable means that you cannot change it. SO, immutability means the inability
// to change a certain variable once it's initialized or once it's declared. You 
// cannot change its value by default. and be careful, i'm talking only about variables.
// I haven't talked about constants yet.
// and this is going to be the next lesson.
// So, when a variable is immutable, its value cannot be changed after its assigned.
// And if you will try to do so, you'll have a compilation error.

fn main() {
    println!("Hello, world!");
    // in rust there is a type annotation
    // let a: u16 = 5;
    // also we can do that like
    let a = 5;
    println!("The value of a is: {}", a);
    //a = 10; // it will generate an error
    // so to resolve this issue we need to make the variable mutable when we declared. `let mut a = 5;`

    // so, lets create another variable
    let mut b = 6;
    println!("The value of b is {}", b);
    b = 3;
}
