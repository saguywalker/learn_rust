pub mod josephus;
pub mod problem1;

use problem1::{sum, dedup, filter, Matrix, mat_mul, sieve};

fn main() {
    josephus::josephus(2, 7);
    println!("{}", sum(&vec![1,2,3,4,5,6,7,8,9]));
    println!("{:?}", dedup(&vec![1,2,2,3,4,5,6,7,7]));
    let even_filtered = filter(&vec![1,2,3,4,5,6], &|i: i32| -> bool {i % 2 ==0});
    println!("{:?}", even_filtered);
    let mat1: Matrix = vec![vec![3.0,4.0,2.0]];
    let mat2: Matrix = vec![vec![13.0, 19.0, 7.0, 15.0],
                            vec![8.0, 7.0, 4.0, 6.0],
                            vec![6.0, 4.0, 0.0, 3.0]];
    println!("{:?}", mat_mul(&mat1, &mat2));
    println!("{:?}", sieve(25));
}
