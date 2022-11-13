trait A {
    fn fa(&self);
}

trait B {
    fn fb(&self);
}

struct C {}
struct E {}
struct D {}

macro_rules! impl_A {
    (for $($t:ty),+) => {
        $(impl A for $t {
            fn fa(&self) {
                println!("This is implement trait A")
            }
        })*
    }
}


macro_rules! impl_B {
    (for $($t:ty),+) => {
        $(impl B for $t {
            fn fb(&self) {
                println!("This is implement trait B")
            }
        })*
    }
}


impl_A!(for D, C, E); 
impl_B!(for D, C, E); 

fn main() {
    let c = C{};
    let e = E{};
    let d = D{};
    c.fa();
    c.fb();
    e.fa();
    e.fb();
    d.fa();
    d.fb();

}