fn main() {
    let vector = vec!{1, 2, 3};
    let x: Option<i32> = get_from_vect(vector, 20);
    
    match x {
        Some(i) => println!("{}", i),
        None => println!("None"),
    };
    
}

fn get_from_vect(vector: Vec<i32>, pos: usize) -> Option<i32> {

    if vector.len() < pos {
        None
    } else {
        Some(vector[pos])
    }
}
