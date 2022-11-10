struct ABC {}

trait A {
    fn f1(&self);
}

trait B {
    fn f2(&self);
}

trait C {
    fn f3(&self);
}

impl A for ABC {
    fn f1(&self) {
        println!("This function f1")
    }
}

impl B for ABC {
    fn f2(&self) {
        println!("This function f2")
    }
}

impl C for ABC {
    fn f3(&self) {
        println!("This function f3")
    }
}


fn main(){
    let abc = ABC{};
    abc.f1();
    abc.f2();
    abc.f3();
}