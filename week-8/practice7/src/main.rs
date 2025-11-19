fn main() {
    // initialization of tuple with data type
    let data_type_tuple:(&str, f32, u8) = ("Rust", 3.14, 100);
    println!("Tuple contents = {:?}", data_type_tuple);

    // initialization of tuple without data type
    let no_data_type = ("Rust", "fun", 100);
    println!("Tuple contents = {:?}", no_data_type);

    // accessing tuple elemrnt at index 0
    println!("Value at Index 0 = {}", data_type_tuple.0);

    // accessing tuple element at Index 1
    println!("Value at Index 1 = {}", data_type_tuple.1);

    // accessing tuple element at index 2
    println!("Value at Index 2 = {}", data_type_tuple.2);
}
