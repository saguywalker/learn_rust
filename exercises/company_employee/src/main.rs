use std::io;
use std::collections::HashMap;

fn main() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    let mut input = String::new();
    println!("Type 'Add <name> to <dept> to add an employee'");
    println!("Type 'List <dept>' to show all employees from dept");
    println!("Type 'All' to show all employees from company");
    println!("Type 'Quit' to quit");

    loop{
        input.clear();
        println!("Please type command.");
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        match Cmd::from_input(&input){
            Some(Cmd::Add{dept, name}) => employees.entry(dept).or_insert(Vec::new()).push(name),
            Some(Cmd::List(dept)) => match employees.get(&dept){
                Some(names) => {
                    println!("All employees in {} dept...", dept);
                    for (i, name) in names.iter().enumerate(){
                        println!("{}. {}", i+1, name);
                    }
                }
                None => println!("No employee in {} dept", dept),
            },
            Some(Cmd::All) => {
                let mut count = 1;
                for (dept, names) in &employees{
                    for name in names{
                        println!("{}. {} ({})", count, name, dept);
                        count += 1;
                    }
                }
            },
            Some(Cmd::Quit) => break,
            None => println!("Wrong command!"),
        };

        println!("***********************************************");
    }
}

enum Cmd{
    Add{dept: String, name: String},
    List(String),
    All,
    Quit,
}

impl Cmd{
    fn from_input(s: &str) -> Option<Self>{
        let words: Vec<&str> = s.trim().split_whitespace().collect();
        match words.as_slice(){
            ["Add", name, "to", dept] => Some(Cmd::Add{
                dept: dept.to_string(),
                name: name.to_string(),
            }),
            ["List", dept] => Some(Cmd::List(dept.to_string())),
            ["All"] => Some(Cmd::All),
            ["Quit"] => Some(Cmd::Quit),
            _ => None,
        }
    }
}
