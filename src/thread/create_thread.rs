mod sync_two_threads;
mod tcp_demo;

use std::thread;
use std::time;
const NTHREADS: u32 = 10;
fn main() {
    let mut children = vec![];
    println!("Hello, world!");
    let ten_seconds = time::Duration::from_secs(10);
    for _i in 0..NTHREADS {
        //Thread will start immediately
        children.push(thread::spawn(move || {
            println!("This is thread number {}", _i);
            thread::sleep(ten_seconds);
            println!("this is end of thread number {}", _i);
        }))
    }

    //Main thread will wait for all child threads finished
    for child in children {
        let _ = child.join();
    }
}
