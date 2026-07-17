use advanced_rust::{dangerous, increment_counter_and_print, print_hello_world, split_at_mut};

fn main() {
    let mut normal_value = 5;
    let normal_ref = &normal_value;

    println!("I am dereferencing the reference. Value - {}", *normal_ref);

    // You can create a raw pointer outside an `unsafe` block.
    // But, you cannot dereference it.
    let unsafe_pointer = &raw const normal_value;

    // A mutable pointer can exist at the same time.
    let unsafe_mutable_pointer = &raw mut normal_value;

    // The following code is commented out, because it doesn't work without an `unsafe` block.
    // println!(
    //     "Attempt to dereferene raw pointer. - Value - {}",
    //     *unsafe_pointer
    // );

    unsafe {
        println!(
            "Attempt to dereferene raw pointer. - Value - {}",
            *unsafe_pointer
        );

        // Change the value using the mutable pointer
        *unsafe_mutable_pointer = 6;

        println!(
            "Dereferencing again with the immutable pointer. Value - {}",
            *unsafe_pointer
        );
        println!(
            "Dereferencing again with the mutable pointer. Value - {}",
            *unsafe_mutable_pointer
        );
    }

    // Cannot call an unsafe function like this.
    // dangerous();

    unsafe {
        dangerous();
    }

    // Testing the working of the safe abstraction.
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    println!("{:?}, {:?}", a, b);

    // Testing extern function

    // Not required to be within an `unsafe` block
    println!("The absolute value of -3 is {}", abs(-3));

    unsafe {
        println!("The square root of 16 is {}", sqrt(16.0));
    }

    // Calling functions using static variables

    // This is not unsafe.
    print_hello_world();

    // This is unsafe.
    // SAFETY: This is only called from a single thread in `main`.
    increment_counter_and_print();
    increment_counter_and_print();
    increment_counter_and_print();
}

unsafe extern "C" {
    // We explicitly say it is safe, because we know `abs` does not involve any memory safety issues.
    safe fn abs(input: i32) -> i32;

    fn sqrt(input: f64) -> f64;
}
