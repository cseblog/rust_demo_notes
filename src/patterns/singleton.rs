
struct AA {}

impl AA {
    fn f(&self) {
        println!("Hellow AAA")
    }
}

trait Singleton {

}

fn main() {
    let aa = AA{a: 10};
    aa.f();
}   