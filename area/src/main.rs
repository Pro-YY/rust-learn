fn add(x: i32, y: i32) -> i32 {
    x + y //expression orientation
    // return x + y; // poor style
}

fn wrong() -> ! {
    panic!("The function never returns!");
}

/*
fn foo(a: [T; N]) -> [T; N] {
    println!("len: {}, first: {}", a.len(), a[0]);
    a
}
*/

fn sum(v: &mut Vec<i32>) -> i32 {
    v.push(4);
    println!("{}, {}", v.len(), v[0]);
    let mut a: i32 = 0;
    for i in v {
        //println!("{}", i);
        a = a + *i;
    }
    a
}

trait HasArea {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

#[derive(Debug)]
struct Square {
    x: f64,
    y: f64,
    side: f64,
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

fn print_area<T: HasArea>(shape: T) -> f64 {
    let a = shape.area();
    println!("area: {:?}", a);
    a
}

fn main() {
    let c = Circle{x: 0.0, y: 0.0, radius: 1.0};
    let s = Square{x: 0.0, y: 0.0, side: 1.0};
    println!("area: {:?}", c);
    println!("area: {}", c.area());
    println!("area: {:?}", s);
    println!("area: {}", s.area());
}
