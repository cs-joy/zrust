fn main() {
    let mut s = String::from("Hello world");
    let hello = &s[0..5];
    s.push_str("!");
    drop(s);
}