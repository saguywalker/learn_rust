use std::env;

fn main(){
    let args: Vec<_> = env::args().collect();
    if args.len() == 1{
        println!("Error! Please provide command!");
        return;
    }

    const DIRECTION: [&'static str; 4] = ["East", "North", "West", "South"];
    let mut direction: i32 = 1;
    let mut x = 0i32;
    let mut y = 0i32;

    let mut i: usize = 0;
    let command: Vec<_> = args[1].chars().collect();
    while i < command.len(){
        match command[i]{
            'W' => {
                let (num_size, num_value) = check_num(&command, i+1);

                if direction % 2 == 0{
                    x = x + num_value as i32 * (1 - direction) as i32;
                }else{
                    y = y + num_value as i32 * (2 - direction) as i32;
                }

                i = i + num_size + 1;
                continue
            },
            'R' => direction = sub_mod(direction, 1, 4),
            'L' => direction = (direction + 1) % 4,
            x => println!("Something: {}", x),
        }
        i += 1;
    }

    println!("X: {}, Y: {}, Direction: {}", x, y, DIRECTION[direction as usize]);

}

fn check_num(cmd: &Vec<char>, idx: usize) -> (usize, u32){
    let mut i = 0;
    let mut value = String::new();
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

fn sub_mod(a: i32, b: i32, m: u32) -> i32{
    let mut result = a - b;
    if result < 0{
        result = m as i32 + result;
    }

    result
}