// Generic Data Types! https://doc.rust-lang.org/book/ch10-01-syntax.html
/**
 * We use generics to create definitions for items like function signatures, or structs, which
 * we can then use with many different concrete data types.
 * Let's look at how to define functions, structs, enums and methods using generics.
 * Then, we will discuss how generics affect code perfromance.
 */

// In Function Definitions
/**
 * When we define a function that uses generics, we place the generics, in the signature of the function
 * where we would usually specify the data types of the paramters and return value.
 * Doing so makes our code more flexible and provides more functionality to callers of our function
 * while preventing code duplication.
 * 
 * Continuing with our `largest` function, below program shows two functions that both the largest value
 * in a slice. We'll then combine these into a single function that uses generics.
 */

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

/**
 * The `largest_i32` function is the one we extracted to finds the largest `i32` in a slice.
 * The `largest_char`function finds the largest `char` in a slice. The function bodies have a the same code, 
 * so let's eliminate the duplication by introducing a "generic type parameter" in a single function.
 * 
 * To parameterized the type in a new single function, we need to name the type parameter, just as we
 * do for the value parameters to a function. You can use any identifier as a type parameter name.
 * But we'll use `T` because,
 * by convention, type parameter names in Rust are short, often just one letter,
 * and Rust's type-naming convention is UpperCamelCase. Short for type, `T` is the 
 * defult choice of the most Rust programmers.
 * 
 * When we use a parameter in the body of the function, we have to declare the parameter name in the signature so that 
 * the compiler knows that name means. Similarly,
 * When we use a type parameter name in a function signature, we have to declare the type parameter name before
 * we use it. To define
 * the generic `largest` function, we place type name declarations inside angle brackets `<>`, 
 * between the name of the function and the parameter list, like this:::
 *                                                                   fn largest<T>(list: &[T]) -> &T {}
 * We read this definition as "The function `largest` is generic over some type `T`. " 
 * This function has one parameter named `list`,  which is a slice of value of type `T`. 
 * The `largest` function will return a reference to a value of the same type `T`.
 * 
 * Below we will write the combined `largest` function definition using the generic data type in its
 * signature. we also shows how we can call the function with either a slice of `i32` values or `char` values.
 * Note that this code won't compile yet.
 */
//v1 - error version
// fn largest<T>(list: &[T]) -> &T {
//     let mut largest = &list[0];
//     for item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

//v2 - solution <T: std::cmp::PartialOrd> instead of <T>
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
/**
 * since it will generate an error and the help text mentions `std::cmp::PartialOrd`, 
 * which is a trait, and we are going to talk about traits in the next section. 
 * For now, know that this error states that the body of `largest` won't work for all
 * possible types that `T`. Becase we want to compare values of type `T` in the body,
 * We can only use types whose values can be ordered. To enable comparisons, the standard
 * library has the `std::cmp::PartialOrd` trait that you can implement on types.
 */

fn main() {
    /////////////// First Example ////////////////////
    // let number_list = vec![34, 60, 48, 25, 100, 87];

    // let result = largest_i32(&number_list);
    // println!("The larges number is {result}");

    // let char_list = vec!['i', 'a', 'l', 'm', 'e'];

    // let result = largest_char(&char_list);
    // println!("The largest character is {result}");

    /////////////// Second Example with Generic type////////////////////
    let number_list = vec![34, 60, 48, 25, 100, 87];

    let result = largest(&number_list);
    println!("The larges number is {result}");

    let char_list = vec!['i', 'a', 'l', 'm', 'e'];

    let result = largest(&char_list);
    println!("The largest character is {result}");
}