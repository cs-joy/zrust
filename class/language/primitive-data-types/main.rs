// Primitive Data Types/Scalar data types (int, float, bool, char)
//
// source: https://rust-book.cs.brown.edu/ch03-02-data-types.html
// source: https://www.youtube.com/watch?v=rQ_J9WH6CGk
// @author: csjoy
// created: June 22 2026
// ===========================
// Integer
// Rust has signed (+ and -) and unsigned integer (only +) types of different size
// i8, i16, i32, i64, i128: Signed integers.
// u8, u16 u32, u64, u128: Unsigned integers.

fn main() {
    let x: i32 = -24;
    let y: u64 = 83;
    println!("Signed Integer: {}", x);
    println!("Unsigned Integer: {}", y);
// diff bet i32 (32 bits) and i64(64 bits)
// signed varients can stores from [-(2^(n-1)) to 2^(n-1) - 1], where
// n is the number of bits that variant uses.
// and unsigned variants cam stores from [0 to 2^(n)-1]
//range :
// i32 - 2147483647
// i64 - 9223372036854775807
    let e: i32 = 2147483647;  // 2^(32-1)-1
    let i: i64 = 9223372036854775807;  // 2^(64-1)-1
    println!("Maximum value of e: {}", e);
    println!("Maximum value of i: {}", i);
}