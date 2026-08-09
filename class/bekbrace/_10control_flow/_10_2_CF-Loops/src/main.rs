// Rust provides three types of loops
// 1- Loop
// 2- While
// 3- For

// fn main() {
//     // Loop keyword
//     // The loop keyword tells Rust to execute a block of code repeatedly until
//     // you explicitly tells it to stop., So it's unconditional loop. it running
//     // running, running and running....until you tells to stop. let's take an example:
//     loop {
//         println!("Hello World!");
//         break;
//     }
// }

// returning value from loops: // https://doc.rust-lang.org/book/ch03-05-control-flow.html#returning-values-from-loops
// fn main() {
//     let mut counter = 0;
//     let result = loop {
//         counter += 1;
//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("the result is: {}", result);
// }

    // also we can create loop labels.
    // Loops Labels to Disambiguate Between Multiple Loops
    // same as Nested Loops,, it's like Russian nested doll.
    // basically `break` and `continue` statements are going to apply to the innermost loop by default.
    // If we'll see: https://doc.rust-lang.org/book/ch03-05-control-flow.html#disambiguating-with-loop-labels
    // When you deal with nested loop, they typical behavior or the default behavior is that the
    // `break` and `continue` statements are going to apply to the innermost loop (by default)

// let's see without loop label
// infinity loop
// fn main() {
//     let mut count = 0;
//     loop { // outer loop
//         println!("count = {count}");
//         let mut remaining = 10;

//         // inner loop ; break and continue statements are going to apply here (it causes infinity loop)
//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }

//     println!("End count: {count}");
// }

fn main() {
    // let mut count = 0;
    // 'counting_up: loop { // outer loop
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     // inner loop
    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    //     // remaining -= 1;
    // }

    // println!("End count: {count}");

    // While Loop # https://doc.rust-lang.org/book/ch03-05-control-flow.html#streamlining-conditional-loops-with-while
    // let mut number = 3;
    // while number != 0 {
    //     println!("number: {}", number);
    //     number -= 1;
    //     // break;
    // }
    // println!("LIFTOFF!");

    // Looping through a Collection with for loop # https://doc.rust-lang.org/book/ch03-05-control-flow.html#looping-through-a-collection-with-for
    // We can choose to use `while` construct to loop over the elements
    // let a = [10, 20, 30, 40];
    // let mut index = 0;
    // while index < 4 {
    //     println!("The value is: {}", a[index]);

    //     index += 1;
    // }
    // however this approach is error-prone; we could cause the program to panic if the index
    // value or the test condition is incorrect.For example, if you changed the definition of 
    // the a array to have four elements but forgot to update the condition to while index < 4, 
    // the code would panic.
    // It's also slow, because the compiler ads runtime code to perform the conditinal check
    // of whether the index is within the bounds of the array on every iteration through the loop.

    // As a more concise alternative, you can use a for loop and execute some code for each item 
    // in a collection.

    // let a = [10, 20, 30, 40];
    // for element in a {
    //     println!("the value is: {element}");
    // }
    // when we run this code, we'll see the same output as used while loop. More importantly,
    // we have now increased the safety of the code that chance of bugs that might result from 
    // going beyond the end of the array or not going far enough and missing some items.
    // Machine code generated from `for` loops can be more efficient as well because the index
    // doesn't need to be compared to the length of the array at every iteration.

    // The safety and conciseness of `for` loops make them the most commonly used loop 
    // construct in Rust. Even in situations in which you want to run some code a certain number
    // of times, as in the countdown example (above using while), most Rustaceans would use a
    // `for` loop. The way to do that would be to use a `Range`. provided by the standard
    // library, which generates all numbers in sequence starting from one number and ending before
    // another number.

    // Here's what the countdown would like using a `for` loop and another method we've not yet
    // talked abotu `rev`. to reverse the range:
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!");
}

