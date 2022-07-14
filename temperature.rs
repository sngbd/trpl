use std::io;

fn main() {
    println!("Select an option:");
    println!("1. Fahrenheit -> Celsius");
    println!("2. Celsius -> Fahrenheit");
    println!("Type option: ");

    let mut opt = String::new();
    io::stdin()
        .read_line(&mut opt)
        .expect("Failed to read input");

    let mut temp = String::new();

    let opt: i32 = match opt.trim().parse() {
        Ok(opt) => opt,
        Err(_) => 0
    };

    if opt == 1 {
        println!("Fahrenheit: ");
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read input");
        let mut temp: f32 = match temp.trim().parse() {
            Ok(temp) => temp,
            Err(_) => 0.0
        };
        temp = (temp - 32.0) * 5.0 / 9.0;
        println!("Celsius: ");
        println!("{temp}");
    } 
    else if opt == 2 {
        println!("Celsius: ");
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read input");
        let mut temp: f32 = match temp.trim().parse() {
            Ok(temp) => temp,
            Err(_) => 0.0
        };
        temp = (temp * 9.0 / 5.0) + 32.0;
        println!("Fahrenheit: ");
        println!("{temp}");
    }
}
