
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

pub type Matrix = Vec<Vec<f32>>;

pub fn mat_mut(mat1: &Matrix, mat2: &Matrix) -> Matrix{
    if mat1[0].len() != mat2.len(){
        panic!("Columns of mat1 must equal to Rows of mat2");
    }
    
    unimplemented!();
}