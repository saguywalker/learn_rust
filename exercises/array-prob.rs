use std::fmt::Display;

fn main(){
    let list = vec![1,3,3,7];
    println!("{}", is_sort(list));

    println!("Ciphertext = {}", cipher_encrypt("hello test".to_string(), 5));
}

fn is_sort<T: PartialOrd + Display>(list: Vec<T>) -> bool{
    if list.len() == 1{
        return true;
    }

    for (i, item) in list.iter().skip(1).enumerate(){
        if item < &list[i]{
            return false;
        }
    }
    true
}