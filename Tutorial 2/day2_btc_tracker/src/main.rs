/*   🎯 Today's Goals:
Learn basic Rust data types

Understand type inference and type annotations

Do arithmetic operations

Start building a CLI Bitcoin Portfolio Tracker (v0.1)


Rust Data Types

| Type   | Example               | Use Case                         |
| ------ | --------------------- | -------------------------------- |
| `i32`  | `let x: i32 = 10;`    | Integer values                   |
| `f64`  | `let y: f64 = 0.002;` | Floating-point values (decimals) |
| `bool` | `let b = true;`       | True/False conditions            |
| `char` | `let c = '₿';`        | Single Unicode characters        |


Rust defaults to:

i32 for integers

f64 for decimals

*/

fn main() {
    let satoshis: i32 = 100_000;
    let btc: f64 = 0.001;
    let is_rich = false;
    let symbol = '₿';

    println!(
        "You have {} satoshis ({} BTC). Rich? {} {}",
        satoshis, btc, is_rich, symbol
    );

    // 🔹 2. Compound Types

    // Tuples

    let user: (&str, f64) = ("Usman", 0.004);
    println!("{} has {} BTC", user.0, user.1);

    // Arrays

    let sats = [1000, 2000, 5000];
    println!("First: {}, Third: {}", sats[0], sats[2]);

    // 📘 Part 2: Arithmetic Operations

    // +  -  *  /  %

    let btc_balance = 0.0025;
    let btc_price_usd = 62_000.0;

    let value = btc_balance * btc_price_usd;

    println!("BTC Value in USD: ${}", value);

    // 🧠 Concept Highlight: Type Inference

    // let x = 5; // Rust infers x is i32
    // let price = 99.9; // inferred as f64

    // let x: u64 = 50;
}
