use std::{sync::mpsc, thread, time::Duration};

fn main() {
    {
        let (tx, rx) = mpsc::channel();

        let tx1 = mpsc::Sender::clone(&tx);

        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        for received in rx {
            println!("Got:{}", received);
        }
    }
    println!("=========================================");
    {
        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            println!("Here is a vector: {:?}", v);
        });
        handle.join().unwrap();
    }
    println!("=========================================");
    {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from spawn thread", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
        handle.join().unwrap();
        for i in 1..5 {
            println!("hi number {} from main thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    }
    println!("=========================================");
}
