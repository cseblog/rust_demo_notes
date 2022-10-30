use std::{thread::sleep, time::Duration};

fn main() {
    println!("Hello looop");
    //1. Matching number
    let mut i = 10;
    
    // do while 
    let a = loop {
        i += 1;
        if i > 100 {
            break i
        }
    };
    println!("a: {}", a);

    // while {}
    let b = loop {
        if i > 1000 {
            break i
        }
         i += 1;
    };
    println!("b: {}", b);

    // while (true) 
    loop {
        println!("Hello while true");
        sleep(Duration::from_millis(5000));
    }
}