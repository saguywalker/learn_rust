pub mod josephus;
pub mod problem1;

use problem1::{sum, dedup, filter};

fn main() {
    josephus::josephus(2, 7);
    println!("{}", sum(&vec![1,2,3,4,5,6,7,8,9]));
    println!("{:?}", dedup(&vec![1,2,2,3,4,5,6,7,7]));
    let even_filtered = filter(&vec![1,2,3,4,5,6], &|i: i32| -> bool {i % 2 ==0});
    println!("{:?}", even_filtered);
}
