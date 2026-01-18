use std::io;

fn main() {
    
    let mut role = String::new();

    println!("Enter user role:");
    println!("admin, project_manager, employee, customer, vendor");

    io::stdin().read_line(&mut role).expect("Failed");
    let role = role.trim().to_lowercase();

    if role == "admin" {
        println!("\nDATABASE STRUCTURE");

        println!("TABLE: staff");
        println!("id | name | department | salary | age");

        println!("\nTABLE: projects");
        println!("project_id | project_name | duration | department");

        println!("\nTABLE: customers");
        println!("customer_id | name | phone | location");

        println!("\nTABLE: data_plan");
        println!("plan_id | plan_name | price | validity");
    }

    else if role == "project_manager" {
        println!("\nPROJECT TABLE");
        println!("project_id | project_name | duration | department");
    }

    else if role == "employee" {
        println!("\nSTAFF TABLE");
        println!("id | name | department | salary | age");
    }

    else if role == "customer" {
        println!("\nCUSTOMER TABLE");
        println!("customer_id | name | phone | location");
    }

    else if role == "vendor" {
        println!("\nDATA PLAN TABLE");
        println!("plan_id | plan_name | price | validity");
    }

    else {
        println!("Wrong role entered");
    }
}

