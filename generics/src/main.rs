use std::any::type_name;
use std::cmp::PartialOrd;

fn largest_element<T: PartialOrd>(list: &[T]) -> &T {
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

fn main() {
    let nunmber_list = vec![2, 5, 6, 3, 9, 1, 4];

    let largest = largest_element(&nunmber_list);

    println!("The largest number is {largest}");

    let char_list = vec!['s', 'a', 'f', 'h', 'u'];

    let largest = largest_element(&char_list);

    println!("The largest number is {largest}");
}
