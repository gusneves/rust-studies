fn main() {
    let number = 6;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let condition = false;
    let value = if condition { 5 } else { 6 };

    println!("Value = {}", value);

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result = {}, counter {}", result, counter);

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    //for with range of numbers (reversed by the rev function)
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
