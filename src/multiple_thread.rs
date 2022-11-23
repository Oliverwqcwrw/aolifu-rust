use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn multiple_thread_test() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}

#[test]
fn multiple_thread_move_test() {
    let v = vec![1,2,3];
    let handle = thread::spawn(move || {
        println!("there is a vector {:?}", v);
    });
    handle.join().unwrap();
}

#[test]
fn channel_test() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
    });
    let reveive = rx.recv().unwrap();
    println!("Got {}", reveive);
}

#[test]
fn channel_two_test() {
    let (tx,rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let vec = vec![
            String::from("1: hi"),
            String::from("1: from"),
            String::from("1: the"),
            String::from("1: thread")
        ];
        for item in vec {
            tx1.send(item).unwrap();
            thread::sleep(Duration::from_millis(200));

        }
    });
    thread::spawn(move || {
        let vec = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];
        for item in vec {
            tx.send(item).unwrap();
            thread::sleep(Duration::from_millis(200));

        }
    });


    for item in rx {
        println!("Got {}", item);
    }
}

#[test]
fn mutex_test() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    // auto unlock when out of scope
    println!("m = {:?}", m)
}

#[test]
fn mutex_arc_test() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("result is {}", *counter.lock().unwrap())
}











