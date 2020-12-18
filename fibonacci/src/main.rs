use std::io;

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    loop {
        println!("Choose the element of the Fibonacci sequence: ");
        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line!");

        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };
        println!(
            "The {}th element of the Fibonacci sequence is: {}",
            number,
            fibonacci(number)
        );
        break;
    }
}
