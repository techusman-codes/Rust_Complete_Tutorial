// fn main() {
//     println!("Hello, world!");
// }

/*What’s happening?
fn defines a function. main() is the entry point.

println! is a macro (note the !), used to print to the console.

All statements end with a semicolon (;).
*/
// How to run this code?

// fn main() {
//     let name = "Usman";
//     let age = 25;

//     println!("Name: {}", name);
//     println!("Age: {}", age);
// }


fn main() {
    let mut balance = 1000;
    println!("Initial Balance: {}", balance);

    balance += 500;
    println!("Updated Balance: {}", balance);
}
