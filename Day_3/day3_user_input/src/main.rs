// fn main() {
//     let balance = 0.002;

//     if balance >= 1.0 {
//         println!("You're a Bitcoin whale!");
//     } else if balance >= 0.1 {
//         println!("You're a BTC shark.");
//     } else {
//         println!("Stacking sats... stay focused!");
//     }

//      let rating = 3;

//     match rating {
//         1 => println!("Terrible"),
//         2 => println!("Bad"),
//         3 => println!("Okay"),
//         4 => println!("Good"),
//         5 => println!("Excellent"),
//         _ => println!("Invalid rating"), // catch-all
//     }

//     use std::io;


//     let mut name = String::new();

//     println!("Enter your name:");
//     io::stdin().read_line(&mut name).expect("Failed to read input");

//     println!("Hello, {}!", name.trim());

// }


use std::io;

fn main() {
    let mut name = String::new();
    let mut btc_str = String::new();

    println!("Enter your name:");
    io::stdin().read_line(&mut name).expect("Failed to read name");

    println!("Enter your BTC balance:");
    io::stdin().read_line(&mut btc_str).expect("Failed to read BTC");

    // Convert BTC balance string to float
    let btc_balance: f64 = btc_str.trim().parse().expect("Please enter a number");

    let btc_price = 62000.0;
    let total_value = btc_balance * btc_price;

    println!("\nHello, {}!", name.trim());
    println!("BTC Balance: {} BTC", btc_balance);
    println!("BTC Price: ${}", btc_price);
    println!("Total Portfolio Value: ${}", total_value);
}




