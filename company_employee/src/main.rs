use std::io;
use std::collections::HashMap;

fn main() {
    let mut company:HashMap<&str, Vec<&str>> = HashMap::new();
    let mut input = String::new();
    
    loop{
        println!("Add employee(1), search employee from department(2), search all(3)...");
        let input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line!");

        println!("{}",&input.len());
    }
}
