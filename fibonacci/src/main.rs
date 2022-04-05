use std::io;

fn main() {
    loop {
        println!("Enter nth value to find fibonacci number for:");

        let mut nth = String::new();
        io::stdin()
            .read_line(&mut nth)
            .expect("Error reading line.");

        let nth: i32 = nth.trim().parse().expect("Expected i32");
        println!("{}", fibonacci(nth));
    }
}

fn fibonacci(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
