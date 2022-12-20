// Polymorphism is feature, one object can be acted by mulitple 
// of instances

// Polymorphism - Dynamic patch -> trait object: &dyn A
// Generic -> 
// Wrapper Enum....

trait Shape {
    fn draw(&self);
}

struct Square{}

struct Circle {}


impl Shape for Square {
    fn draw(&self) {
        println!("Square draw!");
    }
}

impl Shape for Circle {
    fn draw(&self) {
        println!("Circle draw!");
    }
}

fn check(a: &dyn Shape) {
    a.draw();
}


// Trait object -> dyn <Trait name>
// Can not use directly trait object, 
// Can only use trait object by: brorrowing reference (&object),
// Box<dyn Trait>
// Smart pointer

//Good things: Simpler code
//Bad things: 
// - Low performance then generics due to dynamic dispatch
// - Care about trait safety...etc

fn main () {
    
    let sq = Square{};
    let cr = Circle{};

    // Using trait object
    let tools: Vec<&dyn Shape> = vec![&sq, &cr];
    for i in tools.iter() {
        check(*i)
    }

    tools.into_iter().for_each(|f| check(f));


    // Using box
    let x = Box::new(Square{});
    let y = Box::new(Circle{});

    let arr: Vec<Box<dyn Shape>>  = vec![x, y];
    arr.into_iter().for_each(|f| {f.draw();});

}