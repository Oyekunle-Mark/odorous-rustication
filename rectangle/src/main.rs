struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 50,
        height: 60,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );
}

fn area(dimension: &Rectangle) -> u32 {
    dimension.width * dimension.height
}
