use std::io;

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().to_string()
}

fn main() {

    let lager = vec!["33 Export", "Desperados", "Goldberg", "Gulder", "Heineken", "Star"];
    let stout = vec!["Legend", "Turbo King", "Williama"];
    let non_alcoholic = vec!["Maltina", "Amstel Malta", "Malta Gold", "Fayrouz"];

    let product = read_input("Enter a product name to check its category:");

    let mut category = "Unknown";

    if lager.contains(&product.as_str()) {
        category = "Lager";
    } else if stout.contains(&product.as_str()) {
        category = "Stout";
    } else if non_alcoholic.contains(&product.as_str()) {
        category = "Non-alcoholic";
    }

    println!("\nProduct: {}", product);
    println!("Category: {}", category);
}
