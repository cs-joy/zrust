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
}
