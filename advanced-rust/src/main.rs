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
}

// The following function is an `unsafe` function.
unsafe fn dangerous() {
    println!("Hello, I am not unsafe.");

    let x = 5;
    let y = &raw const x;

    // The unsafe part requires to be wrapped inside an `unsafe` block.
    unsafe {
        println!("Now, I am unsafe. Value - {}", *y);
    }
}
