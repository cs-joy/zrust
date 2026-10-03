// Concrete Types

// "concrete type" is basically just used as a distinction against "type that is provided via a type parameter"
// both are types, but in one case you know exactly which one, and in the other, you don't know which one
// E.g. `u8` is a concrete type, "T in fn foo<T>(v: T);" is a generic type where the function body does not know which one exactly, 
// even though you can call the function with T = u8

// struct Vec<T>: T is a generic type
// let xs: Vec<u32>: u32 is a concrete type

fn lowest(data: &[u8]) -> &u8 {
    let mut lowest = &data[0];
    for item in data {
        if item < lowest {
            lowest = item;
        }
    }
    lowest
}

fn main() {
    let number_list = vec![4,7,3,1,10];
    let result = lowest(&number_list);
    println!("result: {result}");
}
