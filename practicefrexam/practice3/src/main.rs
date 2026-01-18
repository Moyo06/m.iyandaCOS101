fn main() {

    for magic_key in 20..29 {

        if magic_key<=25 {
            continue;
        }
        println!("Key is {}",magic_key-3);
    }
}
