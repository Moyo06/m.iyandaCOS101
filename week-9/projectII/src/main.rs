use std::io;


fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().to_string()
}
fn main() {
   let student_name = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold", "Bianca Edemoh"];
   
   let name = input("Enter your name:");

   let matric_number: &str;  
   let department : &str;
   let level : u64;

if name == "Oluchi Mordi"{
    matric_number = "ACC10211111";
    department = "Accounting";
    level = 300;
}
else if name == "Adams Aliyu" {
    matric_number = "AECO10110101";
    department = "Economics";
    level = 100;
}
else if name == "Shania Bolade" {
    matric_number = "CSC10328828";
    department = "Computer";
    level = 200;
}
else if name == "Adekunle Gold" {
     matric_number = "EEE11020202";
     department = "Electrical";
     level = 200;
}
else if name == "Biance Edemoh" {
    matric_number = "MEE10202001";
    department = "Mechanical";
    level = 100;
}
else {
    println!("Name not found.");
    return;
}

println!("\nStudent Information -----");
println!("Name: {}", name);
println!("Matric number: {}", matric_number);
println!("Your department is: {}", department);
println!("Your level is: {}", level);
} 
