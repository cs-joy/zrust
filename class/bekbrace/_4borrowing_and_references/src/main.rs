// References and Borrowing
// Safety and Performance
// Borrrowing and referenes are powerfull concepts

// In Rust managing memory is crucial for ensuring both safety and perfomance.
// But what the word safety means here??
// and why safety is important in programming language?
// and what is safety to begin with?
//
// As a matter of fact, safety referes to the prevention of certain types of bugs and errors
// that commonly occurs in other languages like C and C++. Things like:
// -- null pointer dreferencing
// -- dangling pointers
// -- buffer overflows and
// -- data races.
//
// We should need to explore all of the 4 things

// Here we will discuss about pointer dereferencing or data races or maybe dangling pointers.
// Understanding  References
// References: Enable you to borrow values without taking ownership. It can be immutable and mutable.
// So, let's try to understand what references is. Actually borrowing and references are the same thing.
// Well, basically you create references by borrowing from the original owner of that value.
// Okay. so we have as we explained before in the last lession (ownership), we have only owner
// which is the variable that has only one value. So you cannot have multiple owners for the same
// value. SO let's say that references in Rust enables you to borrow values without taking the ownership
// and this is very important for efficient memory management. Actually Rust doesn't allow you to have multiple
// owners for the same value. That's why we have to create references by borrowing from the owner.
// Also, it's very important to know that references can be both immutable and mutable. 
// So, 
// immutable references allow you borrowing without modification. 
// and of course, 
// mutable references allows you borrowing with the ability to modify the data.
// and to create reference, simply you will add "&" before the variable you're referring to. (Created Reference by add "&")

// example || immutable reference
// fn main() {
//     let _x = 5;
//     let _r = &_x;

//     // let try to increment the value of `_r`
//     //*_r += 1; // it will generate an error of course
    
//     println!("value of _x: {}", _x);
//     println!("value of _y: {}", _r);
// }

// example || mutable reference
// fn main() {
//     let mut _x = 5;
//     let _r = &mut _x;

//     *_r += 1; // correct way to increment value
//     *_r -= 3;

//     println!("value of _x: {}", _x);
//     //println!("value of _r: {}", _r); // will generate an error
//     // because you can have only one mutable reference or many immutable references.
//     // okay! so you can have either one mutable reference to a value any number of immutable references.
// }

// Demonstration on one mutable reference or many immutable references

// STRUCT:
// A data structure that allows you to group multiple fields together under one name.

fn main() {
    // create mutable variable
    let mut account = BankAccount{
        owner: "Alice".to_string(),
        balance: 150.55,
    };

    // now let's do the immutable borrow to check the balance.
    account.check_balance();

    // now let's do the mutable borrow to withdraw money.
    account.withdraw(45.5);

    // again do the immutable borrow to check the balance.
    account.check_balance();

    // so question is why this has successfully compiled? We have said that
    // you can have only one mutable borrow or many immutable borrows. But you can't
    // have both at the same time.
    // The reason why the code compiled successfully because each borrow the check balance and withdraw
    // here in both functions each is in its own scope. So they do not overlab and therefore
    // there is no simultaneous mutable and immutable borrowing of account.

    // soooo., that's all about about borrowing and references!!
}

struct BankAccount {
    owner: String,
    balance: f64,
}

// withdraw money for that BackAccount
impl BankAccount {
    // so,, in this withdraw function. i want to ensure that we can't simultaneously have 
    // "mutable access"
    // to the account that account to update the balance. and
    // for reading the owner's name for example.

    // so how to do that? SImply i'm going to add here 
    // mutable reference to self 
    // also 
    // amount f64 as input as a parameter.
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        // decrement the amount of withdrawn from the account
        self.balance -= amount;
    }

    // to check balance
    // so here i also want to ensure that while we are checking the balance which has immutable access, 
    // no other part of our code modifying the balance which has immutable access. right?
    // so the way to implemnent that we are going to simply print..
    fn check_balance(&self) {
        println!("Account ownder by {} has a balance have {}", self.owner, self.balance);
    }
}