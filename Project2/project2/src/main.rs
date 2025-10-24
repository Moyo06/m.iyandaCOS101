use std::io;

fn main() {
    let mut exp = String::new();
    let mut age = String::new();

    println!("Is the worker experienced? (yes/no): ");
    io::stdin().read_line(&mut exp).expect("Failed to read input");
    let exp = exp.trim();

    println!("Enter the worker's age: ");
    io::stdin().read_line(&mut age).expect("Failed to read input");
    let age:i32 = age.trim().parse().expect("Enter a valid number");

    let pay:i32;

    if exp == "yes" && age >= 40 {
        pay = 1_560_000;
    } else if exp == "yes" && age >= 30 && age < 40 {
        pay = 1_480_000;
    } else if exp == "yes" && age < 28 {
        pay = 1_300_000;
    } else {
        pay = 100_000;
    }

    println!("The total amount is {}", pay);
}

