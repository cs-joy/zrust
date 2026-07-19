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

    // Slice String
}
