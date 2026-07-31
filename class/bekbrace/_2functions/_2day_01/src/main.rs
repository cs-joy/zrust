// Functions
//any function / variables should be written in snake case
// snake case: hello_world
fn main() {
    hello();
    tell_height(182);
    human_id("Alice", 55, 128.0);
    // expression
    let _x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty // Notice that this is the last line in this expression. we won't use any semicolon here. why?
        // because any expression that evaluates to a certain value, a certain mathematical operation
        // will evaluate to the last line in that expression. So if you will leave it like that, price multiply
        // quantity, automatically it's going to evaluate to 50. SO this is a very unique feature in Rust.
        // Alternatively, you can do like that. You can return price multiply by quantity and you can close 
        // it with semicolon (return price * qty;)
        // 
    };
    println!("Result is: {}", _x);

    let y: i32 = add(4,8);
    println!("Value of y: {}", y);
    println!("Value from function 'add' is: {}", add(4,8));

    // calling BMI function
    let weight: f64 = 70.0;
    let height: f64 = 1.82;
    let bmi: f64 = calculate_bmi(weight, height);
    println!("You BMI is: {:.2}", bmi);
}

// Hoisting supported by Rust but many programming languages don't supported it. 
// Hoisting means you can write functions above or bellow of the main function.
fn hello() {
    println!("Hello, Rust!");
}

fn tell_height(height: u32) {
    println!("My height is: {}", height);
}

fn human_id(name: &str, age: u32, height: f32) {
    println!("My name is {}, I am {} years old, and my heigh is {} cm", name, age, height);
}

// also function returning values
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Expressions and Statements
// Expression: Anything that returns a value
// Statement: Anything that doesn't return a value

// Expression
// ends without semcolon in rusty, look at the body of add() function which is an example of expression
//-----------
// 5
// true & false
// add(3,4)
// if condition {value1} else {value2}
// ({code})

// Statements: almost always end with semicolon ;
// let y = let x = 10;
// 1 Variable declarations: let x = 10;
// 2 Function definitions: fn foo() {}
// 3 Control flow statements: if condition { /* code */ }
// else { /* code */ }, while condition { /* code */ }, etc.

// Final example
// BMI = height(kg)/height(m)^2
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}