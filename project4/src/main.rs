use std::io;

fn read_value(prompt: &str) -> f32 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("oops, invalid input");
    let value: f32 = input.trim().parse().expect("invalid input");
    value
}

fn main() {
    loop {
        println!("\nSELECT A SHAPE TO CALCULATE:");
        println!("1. Area of Trapezium");
        println!("2. Area of Rhombus");
        println!("3. Area of Parallelogram");
        println!("4. Area of Cube");
        println!("5. Volume of Cylinder");

        println!("Enter your choice (1-5):");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read");
        let choice: u32 = choice.trim().parse().expect("Invalid choice");

        if choice == 1 {
            let a = read_value("Enter side a:");
            let b = read_value("Enter side b:");
            let h = read_value("Enter height:");
            let area = 0.5 * (a + b) * h;
            println!("Area of Trapezium = {}", area);
        }
        else if choice == 2 {
            let d1 = read_value("Enter diagonal 1:");
            let d2 = read_value("Enter diagonal 2:");
            let area = 0.5 * d1 * d2;
            println!("Area of Rhombus = {}", area);
        }
        else if choice == 3 {
            let b = read_value("Enter base:");
            let h = read_value("Enter height:");
            let area = b * h;
            println!("Area of Parallelogram = {}", area);
        }
        else if choice == 4 {
            let a = read_value("Enter side length:");
            let area = 6.0 * a * a;
            println!("Area of Cube = {}", area);
        }
        else if choice == 5 {
            let r = read_value("Enter radius:");
            let h = read_value("Enter height:");
            let volume = (22.0/7.0) * r * r * h;
            println!("Volume of Cylinder = {}", volume);
        }
        else {
            println!("Invalid selection!");
        }

        println!("\nDo you want to calculate again? (yes/no):");
        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("Failed to read");
        if again.trim().to_lowercase() != "yes" {
            break;
        }
    }
}