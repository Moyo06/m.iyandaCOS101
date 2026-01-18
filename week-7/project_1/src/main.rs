use std::io;

fn shapes(prompt: &str) -> f32 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("oops, invalid input");
    input = input.trim().parse().expect("invalid input");
}
fn solve_the_problem() {
    let aot = "Area of a Trapezium";
    let aor = "Area of a Rhombus";
    let aop = "Area of Parallelogram";
    let aoc = "Area of Cube";
    let voc = "Volume of a Cylinder";
    println!("{}, \n{}, \n{}, \n{}, \n{}", aot, aor, aop, aoc, voc);

    println!("Please, enter an equation");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("oops, inva;id string");
    let input1 =input1.trim().parse().expect("invalid input");

    if input1 == aot {
        let a = shapes("Enter the value of a (length):");
        let b = shapes("Enter the value of b (length):");
        let h = shapes("Enter value of height:");
        let areaoft = 0.5 * (a + b) * h;
        println!("Area of a Trapezium = {}", areaoft);
    }
    else if input1 == aor {
        let d1 = shapes("Enter the first diagonal(length):");
        let d2 = shapes(" Enter the second diagonal(length):");
        let areaofr = 0.5 * d1 * d2;
        println!("Area of a Rhombus = {}", areaofr);
    }
    else if input1 == aop {
        let bp = shapes("Enter the breadth:");
        let hp = shapes("Enter the height:");
        let areaofp = bp * hp;
        println!("Area of a Parallelogram = {}", areaofp); 
    }
    else if input1 == aoc {
        let a = shapes("Enter the length of the side:");
        let areaofc = 6.0 * a.powf(2.0);
        println!("Area of a Cube = {}", areaofc);
    }
    else if input1 == voc {
        let r = shapes("Enter the radius:");
        let hv = shapes("Enter the height:");
        let vofc = 22.0/7.0 * r.powf(2.0) * hv;
    }
    else {
        println!("Don't know what you're talking about!");
    }
}
fn main() {
    loop {
        println!("Hi, there. This is a program designed to 
            help you calculate the area and volume of shapes (some shapes)");
        solve_the_problem();

        println!("Do you want go again?");
        let mut again = String::new();
        io::stdin().read_line(&mut again).expect("oops, invalid string");
        let again = again.trim().to_lowercase();
        if again != "yes" {
            break;
        }
   }
}

