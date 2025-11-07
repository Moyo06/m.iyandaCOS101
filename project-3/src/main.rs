use std::io;

fn main() {
    loop {
        println!("\nFOOD MENU");
        println!("P. Poundo Yam/Edinkaiko Soup");
        println!("F. Fried Rice & Chicken");
        println!("A. Amala and Ewedu Soup");
        println!("E. Eba & Egusi Soup");
        println!("W. White Rice & Stew");

        // Input item code
        println!("Enter item code (P, F, A, E, W):");
        let mut code = String::new();
        io::stdin().read_line(&mut code).expect("Failed to read input");
        let code = code.trim();

        // Match code with item name and price
        let (item_name, price) = match code {
            "P" => ("Poundo Yam/Edinkaiko Soup", 3200.0),
            "F" => ("Fried Rice & Chicken", 3000.0),
            "A" => ("Amala and Ewedu Soup", 2500.0),
            "E" => ("Eba & Egusi Soup", 2000.0),
            "W" => ("White Rice & Stew", 2500.0),
            _ => {
                println!("Invalid code!");
                continue;
            }
        };

        // Input quantity
        println!("Enter quantity:");
        let mut qty = String::new();
        io::stdin().read_line(&mut qty).expect("Failed to read qty");
        let qty: f64 = qty.trim().parse().expect("Invalid quantity");

        // Calculate total
        let mut total = price * qty;

        if total > 10000.0 {
            let discount = total * 0.05;
            total -= discount;
            println!("\n5% discount applied!");
        }

        println!("\nItem: {}", item_name);
        println!("Quantity: {}", qty);
        println!("Final Amount: ₦{:.2}", total);

        println!("\nDo you want to buy another item? (y/n): ");
        let mut ans = String::new();
        io::stdin().read_line(&mut ans).expect("Failed to read");
        if ans.trim() == "n" {
            println!("Program ended");
            break;
        }
    }
}
