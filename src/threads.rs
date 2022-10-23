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

fn shared_reference() {
    // I have a string and I want to share it between the current thread
    // and a new thread. Can I do that with a reference?
    let string = "hello".to_string();

    // std::thread::spawn(|| {
    //     println!("string {}", &string);
    // });

    // No!
    // When we cross thread boundaries you can't guarantee the thread
    // that owns the memory will outlive the thread the references it
    // this could lead to a "use after free"

    // When crossing thread boundaries references have lifetimes of 'static
}

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

/// Fundamental rust concept here
///
/// Functionality is often achieved by type encapsulation
///
/// Each type wrapper adds a specific functionality building upon
/// the functionality of the inner type
///
/// Actual code from roslibrust:
///
/// // We wrap a generic Tcp stream, in a type wrapper that handles encryption (optionally)
/// // and then wrap it in a WebSocketStream to get to a higher level api
/// type Socket = tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<TcpStream>>;
/// // We then wrap our socket in two types that split the read and write halves of the API apart
/// type Reader = SplitStream<Socket>;
/// type Writer = SplitSink<Socket, Message>;
///
/// struct Client {
///   reader: RwLock<Reader>,
///   writer: RwLock<Writer>,
/// }
///
/// type ClientHandle = Arc<Client>;
///
///
/// TcpStream gives us basic read write
/// MaybeTlsStream wraps that with encryption support
/// WebSocketStream gives us a higher level api
/// SplitStream and SplitSink allow us to have one socket but separate Read and Write
/// RwLock is a mutex that allows us to safely read / write from different threads
/// Arc guarantees that the client stays alive wherever it is used
///
/// These types come from 4 different libraries (mio, tokio, tungstenite, std),
/// but all interoperate thanks to traits!

fn main() {}
