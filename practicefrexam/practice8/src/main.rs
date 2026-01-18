use std::io;

fn input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Invalid input");
    input.trim().to_string()
}

fn read_number(prompt: &str) -> f32 {
    println!("{}", prompt);
    input().parse().expect("Invalid number")
}

fn grade(total: f32) -> &'static str {
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
    id: u32,
    name: String,
    department: String,
    total: f32,
    grade: &'static str,
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    loop {
        println!("\n--- Enter Student Details ---");

        println!("Enter Student ID:");
        let id: u32 = input().parse().expect("Invalid ID");

        println!("Enter Name:");
        let name = input();

        println!("Enter Department:");
        let department = input();

        let test1 = read_number("First Test (15):");
        let test2 = read_number("Second Test (15):");
        let participation = read_number("Participation (5):");
        let exam = read_number("Exam (65):");

        let total = test1 + test2 + participation + exam;
        let final_grade = grade(total);

        students.push(Student {
            id,
            name,
            department,
            total,
            grade: final_grade,
        });

        println!("\nDo you want to add another student? (yes/no)");
        let choice = input();

        if choice.to_lowercase() != "yes" {
            break;
        }
    }

    println!("\n--- ALL STUDENT RESULTS ---");
    for s in &students {
        println!(
            "ID: {} | Name: {} | Dept: {} | Total: {} | Grade: {}",
            s.id,
            s.name,
            s.department,
            s.total, 
            s.grade
        );
    }
}

