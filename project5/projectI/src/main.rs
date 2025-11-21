use std::io;

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().to_string()
}

fn read_number(prompt: &str) ->i32 {
    let input = read_input(prompt);
    input.parse().expect("Invalid number")
}
fn main() {
   
    let aps_1_2 = vec!["Intern", "-", "Paralegal", "Placement"];
    let aps_3_5 = vec!["Administrator", "Research_assistant", "Junior_associate", "Classroom_teacher"];
    let aps_5_8 = vec!["Senior_administrator", "Phd_candidate", "Associate", "Snr_teacher"];
    let el1_8_10= vec!["Office_manager", "Post_doc_researcher", "Senior_associate1_2", "Leading_teacher"];
    let el2_10_13 = vec!["Director", "Senior_lecturer", "Senior_associate3_4", "Deputy_principal"];
    let ses = vec!["CEO", "Dean", "Partner", "Principal"];

    let job = read_input("Enter staff job title");
    let years = read_number("Enter years of experience");

    let level: &str;
    let mut level = "Unknown";

    if years >=1 && years <=2 && aps_1_2.contains(&job.as_str()) {
        level = "APS 1-2";
    }
    else if years >=3 && years <=5 && aps_3_5.contains(&job.as_str()) {
        level = "APS 3-5";
    }
    else if years >=5 && years <=8 && aps_5_8.contains(&job.as_str()) {
        level = "APS 5-8";
    }
    else if years >=8 && years <=10 && el1_8_10.contains(&job.as_str()) {
        level = "EL1 8-10";
    }
    else if years >=10 && years <=13 && el2_10_13.contains(&job.as_str()) {
        level = "EL2 10-13";
    }
    else if ses.contains(&job.as_str()) {
        level = "SES";
    } 

     println!("\n---------");
     println!("Staff job title: {}", job);
     println!("Years of experience: {}", years);
     println!("Staff level: {}", level);
}     
    

    


