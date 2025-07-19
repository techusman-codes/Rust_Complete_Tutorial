# Day 2 – Bitcoin Portfolio Tracker (Rust)

This is a simple Rust CLI project to calculate the total value of a user's Bitcoin portfolio in USD.

## 📦 Project Name
`day2_btc_tracker`

## 🚀 What It Does
- Stores user's name
- Stores user's Bitcoin balance
- Multiplies the BTC balance with a fixed BTC/USD price
- Displays the total portfolio value in USD

## 🧠 Concepts Covered
- Rust scalar types (`i32`, `f64`, `bool`, `char`)
- Tuples and arrays
- Arithmetic operations
- String formatting with `println!`
- Type inference and annotations

## 🛠 How to Run

### Step 1: Clone or Create the Project
If creating fresh:
```bash
cargo new day2_btc_tracker
cd day2_btc_tracker

cargo run


fn main() {
    let name = "Usman";
    let btc_balance: f64 = 0.0035;
    let btc_price_usd: f64 = 62000.0;

    let total_value = btc_balance * btc_price_usd;

    println!("User: {}", name);
    println!("BTC Balance: {}", btc_balance);
    println!("BTC Price: ${}", btc_price_usd);
    println!("Total Value: ${}", total_value);
}


