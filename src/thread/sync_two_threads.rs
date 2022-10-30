use std::process::exit;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::mpsc;
use std::thread;
use std::thread::sleep;

/**
Thread A -> send 10 events to Thread B, and wait
Thread B -> consume 10 events from Thread A, and notify
 */
fn main() {
    let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();

    let thread_A_tx: Sender<String> = tx.clone();
    let thread_A_handler = thread::spawn(move || {
        for i in 0..10 {
            thread_A_tx.send(format!("Hello {}", i));
        }
        thread::park();
    });

    let thread_B_handler = thread::spawn(move || {

        for msg in rx {
            println!("{}", msg);
        }
        thread_A_handler.thread().unpark();
    });

    thread_B_handler.join();
}