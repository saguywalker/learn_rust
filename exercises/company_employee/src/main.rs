use std::io;
use std::collections::HashMap;

fn main() {
    let mut company:HashMap<&str, Vec<&str>> = HashMap::new();
    let mut input = String::new();
    
    loop{
        println!("Add employee(1), search employee from department(2), search all(3)...");
        input.clear();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line!");
        input = input.trim().to_string();
        println!("You entered {}", input);
        match input.as_ref(){
            "1" => add_employee(),
            "2" => search_from_dept(),
            "3" => search_all(),
            "Q" => break,
            _ => {
                println!("Please enter only 1,2,3 and Q!")
            }
        };
        
        println!("*************************");
    }
}

fn add_employee() {
    println!("Please enter data...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line!");
    println!("You entered {}", input);
}

fn search_from_dept(){
    println!("Please enter department...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line!");
    println!("You entered {}", input);
}

fn search_all(){
    println!("Search all.");
}
