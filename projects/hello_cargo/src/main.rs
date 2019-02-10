fn main() {
    println!("Hello, world!");

    let mut counter = 0;

    let result = loop{
        counter += 1;

        if counter == 10{
            break counter * 2;
        }
    };

    println!("Counter is {}", result)
}
