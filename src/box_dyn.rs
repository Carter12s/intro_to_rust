#![allow(unused)]
use std::fmt::Display;

/// Now that we understand types and traits let's talk about Sized, Box and References

/// There are two ways to be generic in rust
/// "static dispatch" = what type will actually be used is determined at compile time
/// "dynamic dispatch" = what type will actually be used is determined at run time

// Static dispatch example
fn print_anything<T: Display>(thing: &T) {
    println!("{thing}");
}

// Dynamic dispatch example
// dyn = Dynamic
fn print_anything_dynamic(thing: &dyn Display) {
    println!("{thing}");
}

// When using dynamic dispatch a "&dyn Trait" is implemented
// as a "fat pointer", first a pointer to the actual object
// and second a pointer to the vtable for that types impl
// of that trait. Slower than static dispatch, but required
// in many circumstances

// struct PrintQueue {
//     queue: Vec<&dyn Display>,
// }

// This version of print queue uses a reference to hide the fact
// that we don't know the size of "dyn Display" in this version
// we would have to use some other mechanism to manage lifetime
// of the data and ensure that the PrintQueue releases the reference
// before the memory is dropped
struct PrintQueue<'a> {
    queue: Vec<&'a dyn Display>,
}

// Box is the "owned" equivalent that lets hide the size of something
// Box is a pointer to the heap where that object is so Box<T> always
// has the same size regardless of T
// when the Box is dropped the heap memory is deallocated
// Box == unique_ptr
struct BoxPrintQueue {
    queue: Vec<Box<dyn Display>>,
}

trait Printer {
    fn print(&self);
    // fn add(&mut self, item: &dyn Display);
    // fn push(&mut self, item: Box<dyn Display>);
}

impl<'a> Printer for PrintQueue<'a> {
    fn print(&self) {
        for i in &self.queue {
            println!("{i}");
        }
    }

    // fn add(&mut self, item: &dyn Display) {
    //     self.queue.push(item);
    // }

    // fn push(&mut self, item: Box<dyn Display>) {
    //     self.queue.push(item);
    // }
}

impl Printer for BoxPrintQueue {
    fn print(&self) {
        for i in &self.queue {
            println!("{i}");
        }
    }

    // fn add(&mut self, item: &dyn Display) {
    //     self.queue.push(item);
    // }

    // fn push(&mut self, item: Box<dyn Display>) {
    //     self.queue.push(item);
    // }
}

fn main() {
}
