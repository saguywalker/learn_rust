use std::io;
use std::collections::HashSet;

fn main(){
    println!("Enter number...");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .unwrap();
    input = input.trim().to_string();
    let number: u32 = input
        .parse()
        .unwrap();
    println!("You entered {}", number);
    
    let mut set = HashSet::new();
    for b in 1..number/2{
        for a in 1..b{
            let c2 = a*a + b*b;
            let c = number - a - b;
            if c*c == c2{
                set.insert(vec![a, b, c]);
            }
        }
    }

    for (i, s) in set.iter().enumerate(){
        println!("{}.) {:?}",i+1, s);
    }
}
