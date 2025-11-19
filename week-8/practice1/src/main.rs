fn main() {
   
    // Using Vec::new()
    let V : Vec<i64> = Vec::new();

    // printing the size of vector
    println!("\nThe length of Vec::new is: {}",V.len());

    // Using macro
    let V = vec!["Grace","Effiong", "Basil","Kareem","Susan"];

    // printing the size of vector
    println!("\nThe length of vec macro is: { }",V.len());
}
