use std::io;

fn main() {
    let prices = [
        ("P", 3200),
        ("F", 3000),
        ("A", 2500),
        ("E", 2000),
        ("W", 2500),
    ];

    println!("Menu:");
    println!("P = Poundo Yam/Edikaikong Soup - N3,200");
    println!("F = Fried Rice & Chicken - N3,000");
    println!("A = Amala & Ewedu Soup - N2,500");
    println!("E = Eba & Egusi Soup - N2,000");
    println!("W = White Rice & Stew - N2,500");

    let mut total_price = 0;

    loop {
        let mut input = String::new();
        println!("Enter food type (or 'Q' to quit):");
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let choice = input.trim().to_uppercase();

        if choice == "Q" {
            break;
        }

        let mut quantity = String::new();
        println!("Enter quantity:");
        io::stdin().read_line(&mut quantity).expect("Failed to read line");
        let quantity:i32 = match quantity.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match prices.iter().find(|&&x| x.0 == choice) {
            Some(&(_, price)) => {
                total_price += price * quantity;
            }
            None => {
                println!("Invalid food type. Please select a valid option.");
            }
        }
    }

    if total_price > 10000 {
        let discount = (total_price as f32) * 0.05;
        total_price -= discount as i32;
    }

    println!("Total charges for the order: N{}", total_price);
}