use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread::{sleep};
use std::time::Duration;
use rand::Rng;


fn main() {
    //Create two thread A, Thread B
    // Thread A, waiting for Thread B finish to completed
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    let thread_tx = tx.clone();
    let producer_1 = thread::spawn(move || {
        let mut rng = rand::thread_rng();
        for i in 0..5 {
            sleep(Duration::from_secs(2));
            let n1: u32 = rng.gen();
            thread_tx.send(format!("Thread 1: {}", n1)).unwrap();
        }
    });

    let thread_tx2 = tx.clone();
    let producer_2 = thread::spawn(move || {
        let mut rng = rand::thread_rng();
        for i in 0..10 {
            sleep(Duration::from_secs(1));
            let n1: u32 = rng.gen();
            thread_tx2.send(format!("Thread 2: {}", n1)).unwrap();
        }
    });

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("{:?}", msg);
        }
    });

    producer_1.join();
    producer_2.join();
    consumer.join();
}
