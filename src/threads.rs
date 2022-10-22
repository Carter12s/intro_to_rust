#![allow(dead_code)]

use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

/// Rust's type system is aware of thread boundaries
/// There are special traits which make it safe to send data across threads
///
/// Send = This type is safe to send to another thread
/// Sync = This type is safe to share between threads
///
/// "Plain Old Data" = Send
/// Mutex = Sync + Send

fn shared_rc() {
    // Rc is a std type for a basic "shared_ptr"
    // Rc = "Reference Counted"
    // You can take as many handles to the data as you want
    // When the last handle is dropped the data is deallocated

    let shared_string = Rc::new("hello".to_string());

    let closure_1 = |input: Rc<String>| {
        println!("Closure_1 {}", *input);
    };

    // I can pass my shared string into this closure just fine
    closure_1(shared_string.clone());

    // Can I pass it to another thread?
    // std::thread::spawn(move || println!("New thread: {}", shared_string));

    // No! the internal reference count variable is not guarded by a mutex
    // By creating and destroying handles at the same times in different threads I could
    // break the reference count and that would be unsafe!
}

fn shared_arc() {
    // Arc is "atomic reference count"
    // It's reference count is guarded and therefore the type is Send

    let shared_string = Arc::new("hello".to_string());

    let copy = shared_string.clone();
    std::thread::spawn(move || println!("thread_1: {}", copy));

    // But can I modify the contents that the Arc points to?
    // shared_string.push_str(" world!");

    // We would need Arc<Mutex<String>> to do that!
    // Arc = the lifetime will survive all threads
    // Mutex = enforces a single reader / writer
}

fn edit_war() {
    let data = Arc::new(Mutex::new(0));

    let t1_copy = data.clone();
    std::thread::spawn(move || {
        for _i in 0..100 {
            let mut data = t1_copy.lock().unwrap();
            *data += 1;
        }
    });

    let t2_copy = data.clone();
    std::thread::spawn(move || {
        for _i in 0..100 {
            let mut data = t2_copy.lock().unwrap();
            *data -= 1;
        }
    });
}

fn main() {}
