struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

// re-declare the generics after impl
impl<X1, Y1> Point<X1, Y1> {
    // declare generics on fn, because they're only relevant to the method
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// this impl will only be avaliable if both types are f32
impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p1 = Point { x: 5.0, y: 10.4 };
    p1.distance_from_origin();
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
