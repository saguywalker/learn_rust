use std::env;

fn main(){
    let args: Vec<_> = env::args().skip(1).collect();
    if args.len() == 0{
        println!("Error! Please provide command!");
        return;
    }

    let mut direction = 1;
    let mut x = 0u32;
    let mut y = 0u32;

    let mut i: usize = 0;
    let command: Vec<_> = args[0].chars().collect();
    while i < command.len(){
        match command[i]{
            'W' => {
                let (num_size, num_value) = check_num(&command, i+1);

                

                println!("Walk {}!", num_value);
                i = i + num_size + 1;
                continue
            },
            'R' => println!("Turn Right!"),
            'L' => println!("Turn Left!"),
            x => println!("Something: {}", x),
        }
        i += 1;
    }

}

fn check_num(cmd: &Vec<char>, idx: usize) -> (usize, u32){
    let mut i = 0;
    let mut value = String::new();
    let mut num_chars: Vec<char> = Vec::new();
    while i < cmd.len() - idx{
        match cmd[idx+i].to_digit(10){
            Some(n) => {
                value += &n.to_string();
                i += 1;
            },
            None => break,
        }
    }

    (i, value.parse().unwrap())
}