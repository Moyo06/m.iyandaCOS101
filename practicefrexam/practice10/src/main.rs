use std::io;

fn read_number(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed");
    input.trim().parse().expect("Invalid number")
}

fn main() {
    let test1 = read_number("Enter Test 1 score (out of 15):");
    let test2 = read_number("Enter Test 2 score (out of 15):");
    let participation = read_number("Enter Participation score (out of 5):");
    let exam = read_number("Enter Exam score (out of 65):");

    let total = test1 + test2 + participation + exam;

    
    println!("Total score: {}", total);

    let grade = if total >= 70.0 {
        "A"
    } else if total >= 60.0 {
        "B"
    } else if total >= 50.0 {
        "C"
    } else if total >= 45.0 {
        "D"
    } else {
        "F"
    };

    println!("Grade: {}", grade);
}
