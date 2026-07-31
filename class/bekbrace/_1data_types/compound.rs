// @author: csjoy
// @date: 19/07/2026

fn main() {
    // Four Types:
    ///////////////
    
    // Arrays: declaration: [data_type; size]
    // Fixed size collection of the same type
    // data tyepe: integer
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Numbers are: {:?}", numbers);

    // data_type: slice string type
    let book_categories: [&str; 3] = ["science", "arts", "commerese"];
    println!("book categories are: {:?}", book_categories);


    // Tuples: Genius collection of fixed size
    // declaration: (data_type; size)
    //let book: (u32, String, char, f32, bool) = (1, "Numerical Analysis", 'M', 124.849, true); // here "Numerical Analysis" = slice string type not String. due to this it will generate an error and the solution is 'type conversion'
    // do the conversion: "Numerical Analysis".to_string()
    let book: (u32, String, char, f32, bool) = (1, "Numerical Analysis".to_string(), 'M', 124.849, true);
    println!("book: {:?}", book);

    // also within a tuples we can able to define any another compound data type, for example:
    let library = (1, 'Z', 4.31, "The Art of Computer Programming", book, 0.5); // here book is already tuples type (compound)
    println!("library: {:?}", library);
    // or
    let mix = ("Math", 'M', numbers, false); // numbers is already an Array
    println!("mix: {:?}", mix);

    // Slices: 
    // Dynamically sized view into a contagious sequence of elements. In programming "Contagious"
    // is a very well known terms when it comes to memory. For example,
    // We have an array [1,2,3,4,5,6] so contagious means uninterrupted. Adjacent one another 1->2->3->4 and so on
    // So when displaying something in a contagious sequence from like an array
    // the memory doesn't have to jump between memories but rather going one next to two, element two,
    // the third element, the fourth element, the fifth element and the sixth element, they are all
    // next to each other. ANd this is a good things for memory allocation and memory efficieny. 

    let number_slices: &[i32] = &[1,2,3,4,5,6,7,8,9];
    println!("number slices: {:?}", number_slices);

    let animal_slices: &[&str] = &["Lion", "Elephant", "Crocodile"];
    println!("animal slices: {:?}", animal_slices);

    let book_slices: &[&String] = &[&"Science".to_string(), &"Math".to_string(), &"English".to_string()];
    println!("book slices: {:?}", book_slices);

    // Strings vs String Slices (&str)
    // String: are growable which means expandable, you can increase or decrease them if we want // enhance they are mutable so we can push or delete them from a certain variable. They are own string type that means they are not borrowed
    // String [ growable, mutable, owned string type]
    let mut stone_cold: String = String::from("Hello, "); // store on the Heap
    stone_cold.push_str("World");
    println!("Stone Cold Says: {}", stone_cold);

    // String Slice (&str): you can modify anything and not growable
    // String Slice [immutable, reference]
    // String Slices are used to reference strings literals or substrings (string objects with out needing to copy or own the data),, the is good for memory efficiency, because you don't have to copy the same variable. 
    // So these string slices are used when you want to work with string data without taking ownership of it.
    // The also have specific size and no number of bytes to the stack.
    // So the stack remembers that very well and reacts very quickly in contrast to the heap which is expandable and you know dynamic.
    // It goes at the runtime in the background and can grow bigger and bigger and bigger and can be slower and slower.
    
    // So that's the main difference between the heap and the stack. So, Stack is quicker, the heap is slower.
    // But the stack can't have immutable data types while the heap can have dynamic mutable data types.
    let string: String = String::from("Hello, Rust!");
    //let slice: &str = &string;
    let slice: &str = &string[0..5];
    println!("slice value: {}", slice);
}

// this following function wil generate an error called "not found in this scope, since this is outside of the main function"
// fn print_slice() {
//     println!("SLICE: {}", slice);
// }
