fn main() {
    let x = "hello";
    {
        let y = "world";
        println!("x: {}, y: {}", x, y);
    } //The beauty thing is y is destroyed after this line,

    //We can't use y either, because y go out of its scope
    println!("x: {}, y: {}", x, y); 
}