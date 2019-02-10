use std::io;

fn main() {
    println!("Fahrenheit to Celsius Program.");

    println!("Input temperature in Fahrenheit");
    let mut temp_f = String::new();

    io::stdin().read_line(&mut temp_f)
        .expect("Failed to read line");

    let temp_f: f32 = temp_f.trim().parse()
        .expect("Please type a number!");

    let temp_c: f32 = (5.0 * (temp_f - 32.0)) / 9.0;

    println!("{} F is converted to {} C", temp_f, temp_c);
}
