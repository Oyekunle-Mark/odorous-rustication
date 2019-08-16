fn main() {
    let rect = (50, 60);

    println!("The area of the rectangle is {} square pixels.", area(rect));
}

fn area(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}
