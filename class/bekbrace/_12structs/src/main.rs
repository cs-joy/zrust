// Structs
// Structs are used to name and package related values similar to tuples.
// https://doc.rust-lang.org/book/ch05-01-defining-structs.html

fn main() {
    // tuple
    let rect = (200, 400);
    
    // struct
    struct Book{
        title: String,
        author: String,
        pages: u32,
        available: bool,
    }

    struct User{
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User{
        active: true,
        username: String::from("someusername"),
        email: String::from("someusername@g.com"),
        sign_in_count: 1,
    };
    
    user1.email = String::from("anotheremail@g.com");

    println!("User email is: {}", user1.email);

    // Return a struct from a function
    fn build_User(email: String, username: String) -> User {
        User{
            active: true,
            username: username,
            email: email,
            sign_in_count: 1,
        }
    }

    // Create instances from other instances
    let user2 = User{
        email: String::from("anotheremail2@g.com"), // set specific different value
        ..user1 // keep other fields values are same as first instance
    };

    println!("user2 email: {}", user2.email);
    println!("user2 sign_in_count: {}", user2.sign_in_count);

    // Tuple Struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let white = Color(255, 255, 255);

    // Unit-Like Struct
    struct AlwaysEqual;

    let subject = AlwaysEqual;


}
