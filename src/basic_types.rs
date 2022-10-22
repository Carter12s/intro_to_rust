#![allow(dead_code)]

use std::fmt::{Debug, Display};

/// Defining two of my own types

/// RequestTime is empty it is an alias of the null type ()
/// You can think of the syntax of the null type as a tuple with 0
/// elements
// struct RequestTime;
// struct RequestTime();
struct RequestTime {}

struct TimeResponse {
    time_in_sec: f64,
}

/// Okay I want to be able to print both of these types?

/// First solution "fully manually"
impl RequestTime {
    fn print(&self) -> String {
        "RequestTime()".to_string()
    }
}

impl TimeResponse {
    fn print(&self) -> String {
        format!("TimeResponse{}", &self.time_in_sec)
    }
}

fn basic_printing() {
    let req = RequestTime {};
    let res = TimeResponse { time_in_sec: 0.0 };
    println!("{} {}", req.print(), res.print());
}

/// There is a better way! By defining a Trait!

// A trait is nothing more than an abstract interface, a list of functions
// that must exist on a type for that type to implement the trait
trait Printable {
    fn print(&self) -> String;
}

// We can now implement that trait for our types like so:
impl Printable for RequestTime {
    fn print(&self) -> String {
        "RequestTime()".to_string()
    }
}

impl Printable for TimeResponse {
    fn print(&self) -> String {
        format!("TimeResponse{}", &self.time_in_sec)
    }
}

// I can now generically work "things that are printable" using this trait
fn generic_print_list() {
    // This is a vector of references to items which implement the trait Printable
    let a: Vec<&dyn Printable> = vec![&RequestTime {}, &TimeResponse { time_in_sec: 0.0 }];
    for item in a {
        item.print();
    }

    // Why did we use a reference will this work?
    // let b: Vec<dyn Printable> = vec![RequestTime {}, TimeResponse { time_in_sec: 0.0 }];
    // "dyn Trait" = doesn't have a size known at compile time!
}

/// When we write templated functions in Rust we write them with "Trait Guards"

// Why won't this compile?
// fn bad_generic_printer<T>(item: T) {
//     let s = item.print();
//     println!("{s}");
// }

/// All three of these are valid syntax styles
fn generic_printer_style_a(item: impl Printable) {
    println!("{}", item.print());
}

fn generic_printer_style_b<T: Printable>(item: T) {
    println!("{}", item.print());
}

fn generic_printer_style_c<T>(item: T)
where
    T: Printable,
{
    println!("{}", item.print());
}

/// What are some useful traits from the rust ecosystem?

// A type which can be constructed to a default value impls std::default::Default
// https://doc.rust-lang.org/std/default/trait.Default.html

// A type which can be printed to a string for display to user impls std::fmt::Display
// https://doc.rust-lang.org/std/fmt/trait.Display.html

// A type which can be print to a string for debug purposes impls std::fmt::Debug
// https://doc.rust-lang.org/std/fmt/trait.Debug.html

// A type which can be cloned impls std::clone::Clone
// https://doc.rust-lang.org/std/clone/trait.Clone.html

fn generic_function_that_does_a_lot<T>(input: T)
where
    T: Clone + Display + Debug + Default, // + Operator when working with traits means "And"
{
    let copy_of_input: T = input.clone();
    let another_new_t = T::default();
    println!("Display formatting of T {}", copy_of_input);
    println!("Debug formatting of T {:?}", another_new_t);
}

/// Okay this generic stuff seems really cool? What more can we do?

/// GENERIC TRAIT IMPLEMENTATIONS!!!!!!

trait SarcasmDisplay {
    fn sarcasm(&self) -> String;
}

// THIS WILL AUTOMATICALLY IMPLEMENT MY TRAIT FOR ANY TYPE THAT IS DISPLAYABLE
impl<T: Display> SarcasmDisplay for T {
    fn sarcasm(&self) -> String {
        format!("<sarcasm>{}</sarcasm>", self)
    }
}

// We just extended what any type that implemented display was capable of!
fn use_sarcasm() {
    let _a = "str?".sarcasm();
    let _b = 1010101.sarcasm();
    // Vec doesn't impl display
    // let x = vec!["🤣", "😊"].sarcasm();
}

// You can only impl a Trait for a Type if either the trait or the type were defined in you module
// impl<T: Display> Display for Vec<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

// You can solve this by the `newtype` paradigm of wrapping a type in a type

struct MyVec<T>(Vec<T>);

// Totally legal
impl<T> Display for MyVec<T>
where
    T: Display,
{
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

fn print_vector_of_sarcasm() {
    let my_vec = MyVec(vec!["💩", "hello 🐙"]);
    println!("{}", my_vec.sarcasm())
}

/// Also you don't have to do any of this!

#[derive(Clone, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct ComplexStruct {
    string: String,
    number: u64,
}

// #[derive()] is a macro that will auto generate implementations for most common Traits

fn main() {
    basic_printing();
    generic_print_list();
}
