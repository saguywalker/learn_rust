use std::collections::VecDeque;

pub fn josephus(m: u32, n: u32) {
    let mut queue: VecDeque<u32> = VecDeque::new();
    for i in 0..n{
        queue.push_back(i);
    }

    while !queue.is_empty(){
        for _ in 0..m-1{
            let value = queue.pop_front().unwrap();
            queue.push_back(value);
        }
        print!("{} ", queue.pop_front().unwrap());
    }
}
