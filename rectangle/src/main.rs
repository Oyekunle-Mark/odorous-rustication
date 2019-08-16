struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 50,
        height: 60,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
}
