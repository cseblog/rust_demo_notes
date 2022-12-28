fn main() {
    println!("hello world");
    let mut v = Vec::new();
    v.push("hello");
    v.push("world");


    println!("{:?}", v);


    let mut vv = Vec::new();
    vv.push(v);


    let ii = vv[0].pop();
    println!("{:?}", ii);
}