use std::io;

fn main() {
    loop {
        println!("Type f or c to choose temperature to convert from: ");

        let mut conversion_type = String::new();
        io::stdin()
            .read_line(&mut conversion_type)
            .expect("Error reading line!");

        let mut input = String::new();
        match conversion_type.as_bytes()[0] {
            b'f' => {
                println!("Enter temperature is Fahrenheit.");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Error reading line!");
                let input: f64 = input.trim().parse().expect("Not a number!");
                println!(
                    "{:.1}\u{2109}  is {:.1}\u{2103}",
                    input,
                    (input - 32.0) * 5.0 / 9.0
                );
            }
            b'c' => {
                println!("Enter temperature is Celsius.");
                io::stdin()
                    .read_line(&mut input)
                    .expect("Error reading line!");
                let input: f64 = input.trim().parse().expect("Not a number!");
                println!(
                    "{:.1}\u{2103}  is {:.1}\u{2109}",
                    input,
                    input * 9.0 / 5.0 + 32.0
                );
            }
            _ => println!("Command not recognised!"),
        }
    }
}
