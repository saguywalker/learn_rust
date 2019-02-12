use std::io;
use std::collections::HashMap;

fn main() {
    let numbers = read_input();
    println!("You entered: {:?}", numbers);
    println!("Mean = {}", calc_mean(&numbers));
    println!("Medium = {}", calc_median(&numbers));
    println!("Mode = {}", calc_mode(&numbers));

}

fn read_input() -> Vec<i32>{
    println!("Enter list of number...");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let mut num_vec: Vec<i32> = Vec::new();

    for num in input.split_whitespace(){
        num_vec.push(num.parse::<i32>()
                     .expect("Please enter only number!"));
    }

    num_vec
}

fn calc_mean(v: &Vec<i32>) -> f32{
    let mut sum = 0.0;
    for num in v.iter(){
        sum += *num as f32;
    }
    sum as f32 / v.len() as f32
}

fn calc_median(v: &Vec<i32>) -> f32{
    let size = v.len();
    if size % 2 == 0 {
        (v[size/2] as f32 + v[(size/2)-1] as f32) / 2.0
    }else{
        v[size/2] as f32
    }
}

fn calc_mode(v: &Vec<i32>) -> i32{
    let mut mode_map: HashMap<i32, u32> = HashMap::new();
    for num in v.iter(){
        *mode_map.entry(*num).or_insert(1) += 1;
    }

    let mut max = -1;
    for num in v.iter(){
        if mode_map.get(num) > mode_map.get(&max){
            max = *num;
        }
    }

    max
}
