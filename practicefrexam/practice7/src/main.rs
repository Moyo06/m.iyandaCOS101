use std::io;
use std::fs::OpenOptions;
use std::io::Write;

fn input() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    s.trim().to_string()
}

fn main() {
    let mut students: Vec<String> = Vec::new();

    loop {
        println!("Name:");
        let name = input();

        println!("Matric No:");
        let matric = input();

        println!("First test:");
        let t1: f32 = input().parse().unwrap();

        println!("Second test:");
        let t2: f32 = input().parse().unwrap();

        println!("Participation:");
        let p: f32 = input().parse().unwrap();

        println!("Exam:");
        let exam: f32 = input().parse().unwrap();

        let total = t1 + t2 + p + exam;

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

        students.push(name.clone());

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("results.txt")
            .unwrap();

        writeln!(
            file,
            "{} | {} | Total: {} | Grade: {}",
            matric, name, total, grade
        )
        .unwrap();

        println!("Saved ✅");

        println!("Add another student? (y/n)");
        if input() == "n" {
            break;
        }
    }

    println!("Done. {} students recorded.", students.len());
}

