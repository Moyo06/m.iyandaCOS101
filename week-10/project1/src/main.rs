struct Laptop {
    model:String,
    number:i32,
    cost:i64,
}

fn main() {
    let lap1 = Laptop {
        model:String::from("HP"),
        number:10,
        cost:650_000,
};
    let lap2 = Laptop {
        model:String::from("IBM"),
        number:6,
        cost:755_000,
};
    let lap3 = Laptop {
        model:String::from("Toshiba"),
        number:10,
        cost:550_000,
};
    let lap4 = Laptop {
         model:String::from("Dell"),
         number:4,
         cost:850_000,
};
let qty: i64 = 3;

    let total_hp = lap1.cost * qty;
    let total_ibm = lap2.cost * qty;
    let total_toshiba = lap3.cost * qty;
    let total_dell = lap4.cost * qty;

    let final_total = total_hp + total_ibm + total_toshiba + total_dell;

    println!("Total cost for buying 3 of each brand is: ₦{}", final_total);
}
