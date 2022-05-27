use std::collections::HashMap;
#[derive(Debug)]

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    println!("----VECTORS----");
    let mut vec : Vec<i32> = Vec::new();
    vec = vec![];
    vec.push(7);
    vec.push(11);
    vec.push(13);

    let second : &i32 = &vec[1]; //First way to reference the value
    
    match vec.get(1){ //Second way to reference the value
        Some(second) => println!("The second element is: {}", second),
        None => println!("There is no second element!")
    }
    
    println!("vec = {:?}", vec);

    for i in &mut vec {
        *i += 50;
    }
    
    println!("vec = {:?}", vec);
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("row = {:?}", row);


    println!("\n----STRINGS----");
    let mut s = String::from("hello, ");
    s.push_str("world!");
    println!("{}", s);

    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");

    let tictactoe = t1 + "-" + &t2 + "-" + &t3;
    println!("{}",tictactoe);
    let tic = &tictactoe[0..3];
    println!("{}", tic);


    println!("\n----HASHMAP----");
    let mut scores = HashMap::new();
    scores.insert("Blue", 15);
    scores.insert("Yellow", 17);
    scores.insert("Green", 13);

    for (key, value) in &scores{
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // entry method -> check if the provided key already exists
        *count += 1;
    }

    println!("{:?}", map);
}





