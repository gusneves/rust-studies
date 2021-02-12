#[derive(Debug)] //enable printing the struct

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

//Multiple impl blocks
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        //implemented fn without self parameter
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let sq = Rectangle::square(4);
    println!("Square = {:?}", sq);

    let rec1 = Rectangle {
        width: 120,
        height: 70,
    };
    let rec2 = Rectangle {
        width: 100,
        height: 50,
    };
    let rec3 = Rectangle {
        width: 130,
        height: 50,
    };
    println!("rec1 is {:#?}", rec1);

    println!("The area of the rectangle is {} px²", rec1.area());

    println!("Can rec1 hold rec2? {}", rec1.can_hold(&rec2));
    println!("Can rec1 hold rec3? {}", rec1.can_hold(&rec3));
}
