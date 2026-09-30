// understanding

// user defined type
struct UserData {
    id: u8,
    username: &'static str,
    full_name: &'static str,
    active: bool
}

// trait
trait User {
    // Associated function signature; `Self` referes to the implementor type
    fn new(id: u8, username: &'static str, full_name: &'static str) -> Self;

    // Method signature;
    fn user_id(&self) -> u8;
    fn u_username(&self) -> &'static str;
    fn user_full_name(&self) -> &'static str;
    fn user_nationality(&self) -> &'static str;

    // Default method definitions
    fn profile(&self) {
        println!("==============\n");
        println!("Profile Information about {}", self.u_username());
        println!("ID: {}\nName: {}\nNationality: {}", self.user_id(), self.user_full_name(), self.user_nationality());
    }
}

// implement `UserData` type
impl UserData {
    fn is_activate(&self) -> bool {
        self.active
    }

    fn activating(&mut self) {
        if self.is_activate() {
            // Implementor methods can use the implementor's trait methods.
            println!("{} is already activated!", self.u_username());
        } else {
            println!("{} is still deactivated! activating your account now...", self.username);
            self.active = true;
        }
    }
}

// implement `User` trat for `UserData`
impl User for UserData {
    fn new(id: u8, username: &'static str, full_name: &'static str) -> UserData {
        UserData {id: id, username: username, full_name: full_name, active: false}
    }

    //
    fn user_id(&self) -> u8 {
        self.id
    }

    //
    fn u_username(&self) -> &'static str {
        self.username
    }

    //
    fn user_full_name(&self) -> &'static str {
        self.full_name
    }

    //
    fn user_nationality(&self) -> &'static str {
        if self.is_activate() {
            "Congratulations! You are the first user in our platform, from BD."
        } else {
            "need to activate before set your nationality."
        }
    }

    // Default trait method can be overriden.
    // fn profile(&self) {
    //     // For example, we can add some quiet contemplation.
    //     println!("Hey {}, Welcome to Trait Exploration! Are you from {}?", self.full_name, self.user_nationality());
    // }
}

fn main() {
    // Type annotation is necessary in this case
    let mut data = <UserData as User>::new(123, "zahangir", "Zahangir");
    data.profile();
    println!("activate?: {}", data.is_activate());
    data.activating();
    data.profile();

    //
    println!("username: {}", data.u_username());
    println!("activate?: {}", data.is_activate());
}