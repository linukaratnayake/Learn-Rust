use std::io;

fn main() {
    println!("1 - C -> F, 2 - F -> C");

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Cannot read line");
    let choice: u8 = choice.trim().parse().expect("Enter a number");

    match choice {
        1 => {
            println!("Enter temperature in Celsius: ");

            let temperature = get_temperature();

            let fahrenheit = temperature * 9.0 / 5.0 + 32.0;

            println!("Temperature in Fahrenheit is: {fahrenheit}");
        }
        2 => {
            println!("Enter temperature in Fahrenheit: ");

            let temperature = get_temperature();

            let celsius = (temperature - 32.0) * 5.0 / 9.0;

            println!("Temperature in Celsius is: {celsius}");
        }
        _ => println!("Wrong choice"),
    }
}

fn get_temperature() -> f64 {
    let mut temp = String::new();

    io::stdin().read_line(&mut temp).expect("Cannot read line");
    let temp: f64 = temp.trim().parse().expect("Enter a number");

    return temp;
}
