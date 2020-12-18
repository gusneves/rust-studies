fn main() {
    //Tuples
    println!("\n\nTUPLES :)\n");
    let tup: (i32, f64, u8) = (500, 8.56, 1);
    let (_x, y, _z) = tup;

    println!("Y on tuple = {}", y);

    let n = ('c', 76, 8.9);
    let c = n.0;
    let seventy_six = n.1;

    println!("c = {}, 76 = {}", c, seventy_six);

    //ARRAYS
    println!("\n\nARRAYS :)\n");

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];

    println!("A = {:?}\nB = {:?}", a, b);

    //FUNCTIONS------
    println!("\n\nFUNCTIONS :D\n");
    let q = {
        let p = 5;
        p + 2 //without semicolon
    };
    println!("The value of q is: {}", q);

    let k = plus_five(14);
    println!("k = {:?}", k);
}
//fn function_name(param: type) -> return type {}
fn plus_five(k: i32) -> i32 {
    k + 5
}
