use std::sync::{ Arc, Mutex };
use std::thread;

pub fn use_mutex() {
    let mtx = Mutex::new(19);
    {
        let mut num = mtx.lock().unwrap();
        *num = 20;
    }
    println!("mtx = {mtx:?}");
}

pub fn sharing_mutex_between_threads() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            println!("Thread {:?} executing.", thread::current().id());
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
