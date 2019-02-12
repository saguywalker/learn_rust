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
    
        match input.as_ref(){
            "1" => add_employee(&mut company),
            "2" => search_from_dept(&company),
            "3" => search_all(&company),
            "Q" => break,
            _ => {
                println!("Please enter only 1,2,3 and Q!")
            }
        };
        
        println!("*************************");
    }
}

fn add_employee(comp:&mut HashMap<&str, Vec<&str>>){
    println!("Please enter data...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line!");
    println!("You entered {}", input);

}

fn search_from_dept(comp: &HashMap<&str, Vec<&str>>){
    println!("Please enter department...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line!");
    println!("You entered {}", input);

}

fn search_all(comp: &HashMap<&str, Vec<&str>>){
    println!("Search all.");
}
