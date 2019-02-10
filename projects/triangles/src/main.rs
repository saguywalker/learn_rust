#[derive(Debug)]

struct Triangle{
    base: u32,
    height: u32,
}

impl Triangle{
    fn area(&self) -> f32{
        self.base as f32 * self.height as f32 / 2.0
    }
}

fn main() {
    let tri1 = Triangle{base: 30, height: 50};

    println!(
        "The area of the triangles is {} square pixels.",
        tri1.area()
    );

    println!("tri1 is {:#?}", tri1);
}

