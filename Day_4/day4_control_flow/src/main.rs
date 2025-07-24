/*  | Concept   | Use Case                                                         |
| --------- | ---------------------------------------------------------------- |
| `if/else` | Decision making                                                  |
| `match`   | Pattern matching / alternatives to switch-case                   |
| `loop`    | Infinite loop (useful for event listening e.g., in Bitcoin node) |
| `while`   | Loop until condition false                                       |
| `for`     | Iterate over a range or collection


/*  | Concept   | Use Case                                                         |
| --------- | ---------------------------------------------------------------- |
| `if/else` | Decision making                                                  |
| `match`   | Pattern matching / alternatives to switch-case                   |
| `loop`    | Infinite loop (useful for event listening e.g., in Bitcoin node) |
| `while`   | Loop until condition false                                       |
| `for`     | Iterate over a range or collection                               |
*/
|
*/
fn main() {
    // IF / ELSE
    let btc_balance = 0.002;
    if btc_balance > 1.0 {
        println!("Whale Wallet 🐋");
    } else if btc_balance > 0.01 {
        println!("Decent stack 💼");
    } else {
        println!("Just stacking sats ⚡");
    }

    // MATCH (Better than switch-case)
    let status_code = 200;
    match status_code {
        200 => println!("OK ✅"),
        404 => println!("Not Found ❌"),
        500 => println!("Server Error 💥"),
        _ => println!("Unknown status 🚧"),
    }

    // LOOP (infinite unless break)
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 3 {
            println!("Loop broke at 3");
            break;
        }
    }

    // WHILE
    let mut attempts = 0;
    while attempts < 3 {
        println!("Attempt {}", attempts + 1);
        attempts += 1;
    }

    // FOR (range iteration)
    for i in 1..=5 {
        println!("Block mined: {}", i);
    }
}
