use std::{ thread::{ self, JoinHandle }, time::Duration };

pub fn spawn_thread() -> JoinHandle<()> {
    let join_handle = thread::spawn(|| {
        for i in 0..10 {
            println!("Hi from thread: {i}, spawned thread!");
            thread::sleep(Duration::from_millis(10));
        }
    });
    join_handle
}

pub fn move_closures() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });
    handle.join().unwrap();
}
