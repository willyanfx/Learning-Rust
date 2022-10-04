use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

use tokio::time::sleep;

extern crate futures;

static CARS: i32 = 5;

fn threads() {
    let mut value = 10;

    let a = thread::spawn(move || {
        value = value + 123;
        println!("Thread A {}", value);
    });
    let b = thread::spawn(move || {
        value = value + 1;
        println!("Thread B {}", value);
    });
    let _ = a.join();
    let _ = b.join();
    println!("Thread principal {}", value);
}

fn channels() {
    let (sender, receiver): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for id in 0..CARS {
        let thread_sender = sender.clone();

        thread::spawn(move || {
            let _ = thread_sender.send(id);

            println!("Car {} finish the race", id);
        });
    }

    let mut ids = Vec::with_capacity(CARS as usize);
    for _ in 0..CARS {
        ids.push(receiver.recv());
    }
    println!("Final Order -> {:?}", ids);
}

async fn async_calculation(n: u64) -> u64 {
    if n == 0 {
        panic!("Zero is not valid!");
    } else if n == 1 {
        return 1;
    }

    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    for _i in 1..n {
        sleep(Duration::from_millis(100)).await;
        sum = last + curr;
        last = curr;
        curr = sum;
    }

    sum
}

async fn example_task() {
    let num1 = 75;
    let num = async_calculation(num1).await;
    println!("{} => {}", num1, num);
}

#[tokio::main]
async fn main() {
    // threads();
    // channels();

    // let future = example_task();
    // for _ in 0..10 {
    //     println!("Nothing happen here");
    // }

    // futures::executor::block_on(future);

    tokio::spawn(async { example_task().await });

    loop {
        sleep(Duration::from_millis(500)).await;
        println!("Nothing happen here");
    }
}
