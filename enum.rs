enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match &self {
            &Message::Write(text) => {
                println!("{}", text)
            }
            &Message::Move { x, y } => {
                println!("({}, {})", x, y);
            }
            &&Message::Quit => {
                println!("Quit");
            }
            _ => {
                println!("Other");
            }
        }
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let penny = Coin::Quarter;
    println!("{}", value_in_cents(penny));
    let message = Message::Quit;
    message.call();

    let opt: Option<i32> = None;
    if let Some(value) = opt {
        println!("Valid value!");
    } else {
        println!("Invalid value!");
    }
}
