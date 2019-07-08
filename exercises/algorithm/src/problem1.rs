
pub fn sum(slice: &[i32]) -> i32{
    slice.iter().fold(0i32, |acc, x| acc + x)
}

pub fn dedup(vs: &Vec<i32>) -> Vec<i32>{
    let mut result: Vec<i32> = Vec::new();
    'outer: for x in vs.into_iter(){
        for y in result.iter(){
            if x == y{
                continue 'outer;
            }
        }
        result.push(*x)
    }
    result
}

pub fn filter(vs: &Vec<i32>, pred: &Fn(i32) -> bool) -> Vec<i32>{
    let mut result: Vec<i32> = Vec::new();
    for v in vs.into_iter(){
        if pred(*v){
            result.push(*v);
        }
    }
    result
}

//matrix multiplication
pub type Matrix = Vec<Vec<f32>>;

pub fn mat_mul(mat1: &Matrix, mat2: &Matrix) -> Matrix{
    if mat1[0].len() != mat2.len(){
        panic!("Columns of mat1 must equal to Rows of mat2");
    }
    let mut mat3: Matrix = vec![vec![0.0; mat2[0].len()]; mat1.len()];
    for i in 0..mat1.len(){
        for j in 0..mat2[0].len(){
            for k in 0..mat2.len(){
                mat3[i][j] += mat1[i][k] * mat2[k][j]
            }
        }
    }
    mat3
}

//sieve of eratosthenes
pub fn sieve(n: u32) -> Vec<u32>{
    let mut result: Vec<u32> = Vec::new();

    let mut is_prime = vec![true; n as usize + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    let mut even = 2 * 2;
    while even < n{
        is_prime[even as usize] = false;
        even += 2;
    }
    result.push(2);
    
    for i in 3..=n{
        if is_prime[i as usize]{
            let mut tmp = i * i;
            while tmp < n{
                is_prime[tmp as usize] = false;
                tmp *= 2;
            }
            result.push(i);
        }
    }

    result     
}

//tower of hanoi
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Peg{
    A,
    B,
    C,
}

pub type Move = (Peg, Peg);

pub fn hanoi(num_discs: u32, src: Peg, aux: Peg, dist: Peg) -> Vec<Move>{
    unimplemented!();
}