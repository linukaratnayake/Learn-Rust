use std::slice;

static HELLO_WORLD: &str = "Hello World!";
static mut COUNTER: u32 = 0;

// The following function is an `unsafe` function.
pub unsafe fn dangerous() {
    println!("Hello, I am not unsafe.");

    let x = 5;
    let y = &raw const x;

    // The unsafe part requires to be wrapped inside an `unsafe` block.
    unsafe {
        println!("Now, I am unsafe. Value - {}", *y);
    }
}

// This is a safe abstraction of unsafe rust.
// The function does the assertion, so that it makes sure the pointer is always valid.
pub fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

pub fn print_hello_world() {
    println!("{}", HELLO_WORLD);
}

/// SAFETY: Calling this function from more than a single thread
/// at a time is undefined behavior.
pub fn increment_counter_and_print() {
    unsafe {
        COUNTER += 1;
        println!("COUNTER - {}", *(&raw const COUNTER));
    }
}
