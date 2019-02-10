use std::io;

fn main() {
    println!("Fibonacci program.");

    println!("Please type number...");
    let mut n = String::new();
    io::stdin().read_line(&mut n)
        .expect("Failed to read line.");

    let n: u32 = n.trim().parse()
        .expect("Please type only number.");

    let result = fib(n);
    println!("Fibbonacci of {} is {}", n, result);
}

fn fib(n: u32) -> u32{
    if n == 0 || n == 1{
        n
    }else{
        fib(n - 1) + fib(n - 2)
    }
}
