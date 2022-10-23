/// A correction from last time on str, slice, and compile time size

fn main() {
    let a_str = "hello world";
    let b_string = a_str.to_string();

    // str is a "slice" containing both a pointer to the memory and the length of the region
    // Both are determined at runtime
    // The size of a str is not known at compile time, because its length is dynamic so we can
    // only pass it by reference (or within some other sized container)
    let sub_str = &b_string[0..3];

    // https://github.com/rust-lang/rfcs/issues/1833
    // let compile_time_sized_str: &[u8; 3] = sub_str.as_bytes()[0..3];
    // let compile_time_sized_str = sub_str.fixed_slice<3>(0);

    println!("{sub_str}");

    let array = [0, 1, 2, 3];

    // Slices in rust are "runtime" size (not mut sized)
    let slice = &array[0..2];

    // [i32] => array of i32 of any length including 0, size unknown
    // &[i32] => a slice of i32, slice itself stores start and length
    // [i32; 3] => a fixed length array of i32

    println!(
        "Size of array {}, Size of slice {}",
        std::mem::size_of_val(&array), // 16 => 4 bytes * 4 values
        std::mem::size_of_val(slice),  // 8 => 64 bit ptr + 64 bit size (on target architecture)
    );

    println!(
        "Address of array {:?}, Address of slice {:?}",
        array.as_ptr(),
        slice.as_ptr(),
    );

    println!(
        "Length of array {}, Length of slice {}",
        array.len(),
        slice.len(),
    );

    // str == [u8] with the restriction that the u8 is valid unicode
    let fancy_str = "💩";
    // let fancy_slice = &fancy_str[0..1];
    let fancy_slice = fancy_str.chars().nth(0).unwrap();
    println!("{fancy_slice}");

    fn takes_slices(some_bytes: &[u8], three_floats: [f32; 3]) {
        println!("{some_bytes:?}, {three_floats:?}")
    }

    println!("{slice:?}");
    takes_slices(&[1, 2, 3, 4, 6, 7], [0.1, 0.2, 0.3]);
}
