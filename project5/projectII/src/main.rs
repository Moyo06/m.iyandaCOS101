use std::io;

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn read_number(prompt: &str) -> i32 {
    let input = read_input(prompt);
    input.parse().expect("Invalid number")
}

fn main () {
 
let mut candidates: Vec<(String, i32)> = Vec::new();

let positions = vec!["first", "second", "third"];

for pos in positions {
    let name = read_input(&format!("Enter name of the {} candidate:", pos));
    let years = read_number(&format!("Enter years of programming experience for {} candidate:", pos));

    candidates.push((name, years));
}

    let mut highest = &candidates[0];

    for candidate in &candidates {
        if candidate.1 > highest.1 {
            highest = candidate;
        }
    }
    
    println!("\nThe candidate with the highest ecperience is {} with {} years", highest.0, highest.1);
    
}