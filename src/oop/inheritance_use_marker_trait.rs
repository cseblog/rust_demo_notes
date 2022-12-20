
trait A {
    fn fa(&self);
}

trait B {
    fn fb(&self);
}

// This is just empty marker trait
trait Common {}

struct C {}
struct D {}
struct E {}
impl Common for C {}
impl Common for D {}
impl Common for E {}


impl <T> A for T
where T: Common {
   fn fa(&self) {
       println!("This is trait A")
   }
}

impl <T> B for T
where T: Common {
    fn fb(&self) {
        println!("This is trait B")
    }
}

fn main() {
    let c = C{};
    c.fa();
    c.fb();
    (&c)

    
    let e = E{};
    e.fa();
    e.fb();
}