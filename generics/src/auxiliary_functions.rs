use std::any::type_name;
use std::cmp::PartialOrd;

pub fn largest_element<T: PartialOrd>(list: &[T]) -> &T {
    let the_type = type_name::<T>();
    println!("The type of T is {the_type}");

    let mut largest = &list[0];

    for element in list {
        if element > largest {
            largest = element;
        }
    }

    largest
}
