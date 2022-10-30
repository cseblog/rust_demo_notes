use std::{ops::{Add, Sub}, fmt::Display};





//Generic Struct
struct GenericT<T> {
    a: T,
    b: T
}

trait Human {
    fn add(&self);
    fn sub(&self);
}

//T implement +/-, Copy  for borrowing, Display for print
impl <T: Add<Output = T> + Sub<Output = T> + Copy + Display> Human for GenericT<T> {
    fn add(&self) {
        let c = self.a + self.b as T;
        println!("{}", c)
    }

    fn sub(&self) {
        let c = self.a - self.b;
        println!("{}", c)
    }
}


// a, b are bounded by Add, Display traits
// or we can say: a, b are having Add, Display traits
fn add<T: Add<Output = T> + Display>(a: T, b: T) {
    println!("{}", a);
    println!("{}", b);
    let c:_ = a + b;
}

// Another way to implement add
// fn add2(a: impl Add<Output = _> + Display, b: impl Add<Output = _> + Display) {
//     println!("{}", a);
//     println!("{}", b);
//     let c:_ = a + b;
// }


fn main() {
    let a = 10; 
    let b = 11;
    add(a, b);
    let x = 10.0;
    let y = 11.1;
    add(x, y);

    // add2(x, y);


    let g = GenericT{a: 100, b: 10};
    g.add();
    let g2 = GenericT{a: 100, b: 222};
    g2.sub()
}