use std::io;

fn main() {
    println!("This program prints the Celcius equivalent of temperatures entered in Fahrenheit.");

    loop {
        println!("Enter temperature in Fahrenheit");
        println!("Enter zero to terminate the program");

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Could not read line");

        let temp = temp.trim().parse::<f64>().unwrap();

        if temp == 0.0 {
            break;
        }

        let fahr = (temp - 32.0) * 5.0 / 9.0;
        println!(
            "The Celcius equivalent of {} Fahrenheit is {} Celcius",
            fahr, temp
        );
    }

    println!("Thank you!");
}
