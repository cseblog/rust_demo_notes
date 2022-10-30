#[derive(Clone, PartialEq, Debug)]
struct Human {
    name: String,
    age: u32
}

#[derive(Debug)]
struct Dog<'a> {
    name: &'a str,
    age: u32
}

fn main() {
    println!("Hello looop");
    //1. Matching number
    let i = 10;
    match i {
        10 => { println!(" We got 10 !")}
        _ => { println!("we got another number")}
    }
    //2. Matching String
    let result = "OK";
    match result {
        "OK" => {println!(" We got an {}", result)}
        _ => {println!("We got an other string")}
    }

    //1. Mactching enum
    let result: Result<&str, i32> = Ok("OK");
    match result {
        Ok(msg) => println!("Function return {}", msg),
        Err(_) => println!("This function is error return")  
    }

    //2. Maching struct
    let dog = Dog{name: "Tom", age: 10};
    match Some(dog) {
        Some(d) if d.name == "Tom" => print!("It is  {:?}", d),
        Some(k) => println!("It is a dog {:?}", k),
        _ => println!("She is not a dog")
    }
}