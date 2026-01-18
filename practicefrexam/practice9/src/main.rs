use std::io;

fn input(prompt: &str) ->
String {
    let mut value = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut value).expect("Failed to read input");
      value.trim().to_string()
}

fn read_number(prompt: &str) -> f32 {
    input(prompt).parse().expect("Invalid number")
}

fn determine_grade(total: f32) -> &'static str {
    if total >= 70.0 {
        "A"
    } else if total >= 60.0 {
        "B"
    } else if total >= 50.0 {
        "C"
    } else if total >= 45.0 {
        "D"
    } else {
        "F"
    }
}


struct Student {
    id: String,
    name: String,
    department: String,
    grade: &'static str,
} 

fn main() {
    println!("--- Welcome to student Information Management System ---");  //printing welcoming message
   
    let id: String = input("Enter your ID: ").parse().expect("Error, re-check it");

    println!("Enter your name:");  //name 
    let name:String = input("Enter Name: "); 

    println!("Enter your Department:"); //department 
    let department: String = input("Enter Department: ").parse().expect("Error, re-check it");

    println!("How many courses are you offering?");
    let course_count: u32 = input("Enter number of courses: ").parse().expect("Invalid number");

       for i in 1..= course_count
    {
        println!("------Course {}------", i);
    

   
    let test1 = read_number("Enter first test score(out of 15):");

    let test2 =read_number("Enter second test score(out of 15):");

   let participation : f32 =read_number("Enter participation mark (out of 5):");

   let exam = read_number("Enter exam scorw (out of 65):");

       let total = test1 + test2 + participation + exam;
       let final_grade = determine_grade(total);

    println!("Total score: {}", total);
    println!("Grade: {}", final_grade);
        } 
    
     
    println!("\n..... Student Details....");
    println!("ID: {}\nName: {}\nDepartment:{}",
        id, name, department); 
        
}
