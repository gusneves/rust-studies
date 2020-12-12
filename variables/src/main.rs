//consts -> used in several parts of the program
const MAX: u32 = 100_000;

fn main() {
    println!("{:?}\n", MAX);

    // Mutable variables--------
    let mut x = 6;
    println!("X = {}", x);
    x = 4;
    println!("X = {}\n", x);

    //Shadowing-----------------
    let y = 4;
    let y = y * 4;
    let y = y * 4;

    println!("Y = {}\n", y);

    // why don't use just 'let mut'?
    let spaces = "   ";
    let spaces = spaces.len(); // new variable with same name, but different type
                               /*
                               let mut spc = "   ";
                               spc = spc.len(); //code panic. different type to same variable
                               */
}
