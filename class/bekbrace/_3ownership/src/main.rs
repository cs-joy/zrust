// Ownership, Borrowing and References

// Ownership:
// ----------
// C, C++ -> Memory Management Control Issue
// Garbage collector solved this issue, but created a new issue
// [stopeping/resuming the program]
// ----------
// First let's understand why ownership exists and the problems it has solved.
// As you know, there are a numebr of programming laguages that let you control the memory
// like C and C++. They allow you to reserve a part of the memory and when you finished using
// this part of memory, it lets you release or free this part of memory. The problem
// with this system that it creates bugs because you might have freed the memory more than once 
// or you even forgot to free that memory that chunk of memory. This is of course for the languages
// that let you totally control the memory. Now this issue is solved by some programming languages
// by some programming languages by introducing the garbage collector.
// Now, the garbage collector has a role of reserving the data into memory and then once the 
// programmer is done dealing with this data the garbage collector releases this part of memory.
// Now this operation is done at the runtime in the background and this is one of the main drawbacks
// of garbage collection. If it wants to clean up the memory it will stop the program. And by
// the way that is the freezing which happens for a few seconds in your program. SO blame it on
// the garbage collector as it stops the program to clean up the memory and when it's done cleaning
// the program resumnes working. So this actually has created a slow performance and inefficient
// outcome not recommended for those applications that need powerful memory resources. 
// Now let us see, 
// how rust has solved this issue by new concept called "ownership"
// look at that: https://bidenwhitehouse.archives.gov/wp-content/uploads/2024/02/Final-ONCD-Technical-Report.pdf
// 
// What is Ownership?
// Every value has a single owner [every variable has one value, and it is it's also sole owner].
// 
//
// Now in Rust, every value has a singler owner and there can only be one owner at a time. Actually
// Ownership rules help manage memory efficiently and prevent common bugs. And as we're talking about
// ownershipm we will have to talk about borrowing and references. 
// So borrowing in nutshell allows you to temporarily borrow references to values. And this actually
// enables safe concurrent access without sacrificing the memory safety.
//
// Now there are very important three rules of ownership and these rules actually are defined here in the Rust book ...
//
// 1. Each value in Rust has an owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.
//
// ... and the book recommends to keep these rules in mind. 
// https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-rules
// You don't have to know them by heart but it's good to keep them in a corner of your mind.

// Now, let me give three examples on the three rules. 
// Example1 || rules 1: Each value in Rust has a variable that's its owner.

// fn main() {
//     let s1 = String::from("RUST");
//     let len = calculat_length(&s1);
//     println!("Length of '{}' is {}", s1, len);
// }

// fn calculat_length(s: &String) -> usize {
//     s.len()
// }

// Example2 || rules 2: There can be only one owner at a time.
// fn main() {
//     let s1 = String::from("RUST");
//     let s2 = s1; // transfer ownership

//     // println!("{}" ,s1); // it will generate an error
//     println!("{}", s2);
// }

// Example3 || rules 3: When the owner goes out of scope, the value will be dropped.
fn main() {
    let s1 = String::from("RUST");
    let len = calculate_length(&s1);
    println!("Length of '{}' is{}", s1, len);
}
// s1 goes out of scope and its value will be dropped
fn printLost(str: &String) {
    println!("{}", s1);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}