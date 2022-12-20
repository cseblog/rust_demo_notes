struct Circle{}
trait Shape {
    fn area(&self) {
        println!("default area");
    }
}

impl Shape for Circle {
    
}

// impl Shape for Circle {
//     fn area(&self) {
//         println!("this new area");
//     } 
// }


impl Circle {
    fn area(&self) {
        println!("This is from constructor")
    }
    fn area(a: u32, b: u32) {
        println!("hello....")
    }
}


fn main() {
    println!("hello world");

    let c = Circle{};
    c.area()
}