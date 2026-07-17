fn main() {
    // Four Types:
    //////////////

    // Integer: signed (i8, i16, i32, i64, i128) and unsigned(u8, u16, u32, u64, u128)
    let x1: i8 = -7;
    let x2: i128 = -64565675;
   
    let y1: u8 = 46;
    let y2: u128 = 37645784;

    println!("value of (x1, y1): ({}, {})", x1, y1);
    println!("value of (x2, y2): ({}, {})", x2, y2);

    // Float: f32 and f64
    let width: f32 = 33.95;
    let height: f64 = 3.876876786;
    println!("width = {}, \nheight= {}", width, height);

    // Boolean: bool
    let is_left: bool = false;
    println!("is_left: {}", is_left);

    // Character: char
    let first_letter_of_the_alphabet: char = 'A';
    println!("first_letter_of_the_alphabet: {}", first_letter_of_the_alphabet);
}