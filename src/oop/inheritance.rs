
trait A {
    fn fa(&self);
    fn fa2(&self);
}


trait B {
    fn fb(&self);
    fn fb2(&self);
}

struct C {}

impl A for C {
    fn fa(&self) {
        println!("This is A, fa");
    }
    fn fa2(&self) {
        println!("This is A, fa2");
    }
}


impl B for C {
    fn fb(&self) {
        println!("This is B, fb");
    }
    fn fb2(&self) {
        println!("This is B, fb2");
    }
}



fn main() {
    let c = C{};
    c.fa();
    c.fa2();
    c.fb();
    c.fb2()



}