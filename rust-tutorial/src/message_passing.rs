use std::sync::mpsc::{ self, Receiver };
use std::thread;
use std::time::Duration;

/// mpsc stands for multiple producer, single consumer
pub fn mpsc_channel_and_message_passing() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        // let value = String::from("Hi");
        // tx.send(value).unwrap();

        let values = vec!["Hi", "from", "the", "thread"];
        for elem in values {
            tx.send(elem).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    thread::spawn(move || {
        // let value = String::from("Hi");
        // tx.send(value).unwrap();

        let values = vec!["Coco", "Zaki", "Thunder", "Akanksha"];
        for elem in values {
            tx1.send(elem).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    thread::sleep(Duration::from_secs(1));

    // let received = rx.recv().unwrap();
    // println!("Got: {received}");

    for received in rx {
        println!("Got: {received}");
    }
}
