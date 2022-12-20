// In java Overloading is child object can call parent's methods
// In rust, we call it as dynamic loading

// Dynamic loading is implemented in two ways
// 1. Using smart pointer: Box
// 2. Using object trait dyn Object

trait Shape {
    fn area(&self) -> u32;
}

struct Circle {
    x: u32,
    y: u32,
}

struct Square {
    x: u32,
    y: u32,
}

impl Shape for Circle {
    fn area(&self) -> u32 {
        println!("This is Circle!");
        return self.x * self.y * self.y;
    }
}

impl Shape for Square {
    fn area(&self) -> u32 {
        println!("This is Square");
        return self.x + self.y;
    }
}

fn main() {
    let c = Circle{x: 100, y: 20};
    let s = Square{x: 200, y: 100};
    
    // let bc:Box<dyn Shape> = Box::new(c);
    // let bs :Box<dyn Shape> = Box::new(s);

    let bct: &dyn Shape = &c;
    let bst: &dyn Shape = &s;
    let l = vec![bct, bst];
    for item in l.into_iter() {
        item.area();
    }
}