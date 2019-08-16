fn main() {
    let width = 50;
    let height = 60;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
