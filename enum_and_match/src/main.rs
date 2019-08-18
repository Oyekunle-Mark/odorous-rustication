#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> i32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let quarter = Coin::Quarter(UsState::Alabama);

    println!("The value of quarter is {}!", value_in_cents(quarter));

    let some_u8_value = Some(3);

    if let Some(1) = some_u8_value {
        println!("Three");
    } else {
        println!("Not a three");
    }
}
