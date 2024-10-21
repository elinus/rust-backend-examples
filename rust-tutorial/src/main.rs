use simple_threading::{ move_closures, spawn_thread };
use std::{ thread, time::Duration };

#[allow(unused)] // For beginning only.
pub mod collections_vector;
pub mod simple_threading;
pub mod message_passing;
pub mod shared_state_concurrency;
pub mod linked_list;

fn main() {
    println!("Hello from Rust, Coco!");

    // test_func();

    collections_vector::test_std_collections_vector();

    // let handle = spawn_thread();
    // for i in 0..5 {
    //     println!("Hi from thread: {i}, main thread!");
    //     thread::sleep(Duration::from_millis(10));
    // }
    // handle.join().unwrap();

    // move_closures();

    // message_passing::mpsc_channel_and_message_passing();

    // shared_state_concurrency::use_mutex();
    // shared_state_concurrency::sharing_mutex_between_threads();
}

// fn test_func() {
//     let x = ();
//     println!("{:?}", x);
// }
