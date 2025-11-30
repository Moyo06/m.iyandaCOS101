use std::io;

fn input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().to_string()
}
fn main() {
    let name_of_commissioner = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu", "Okorocha Calistus Ogbona", "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];

    let name = input("Enter your name:");

    let ministry: &str;
    let geo_political_zone: &str;


if name == "Aigbogun Alamba Daudu" {
   ministry = "Internal Affairs";
   geo_political_zone = "South West";
}

else if name == "Murtala Afeez Bendu" {
    ministry = "Justice";
    geo_political_zone = "North East";
}
else if name == "Okorocha Calistus Ogbonna" {
    ministry = "Defense";
    geo_political_zone = "South South";
}
else if name == "Adewale Jimoh Akanbi" {
    ministry = "Power & Steel";
    geo_political_zone = "South West";
}
else if name == "Osazuwa Faith Etieye" {
    ministry = "Petroleum";
    geo_political_zone = "South East";
}
else {
    println!("Invalid name");
    return;
}

println!("\nInformation of Ministers");
println!("Name: {}", name);
println!("Ministry: {}", ministry);
println!("Geo_political_zone: {}", geo_political_zone);
}