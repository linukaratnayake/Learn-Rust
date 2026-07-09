use core::f64;
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

pub struct Point<T, U>(pub T, pub U);

impl Point<f64, f64> {
    pub fn distance_from_origin(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2)).sqrt()
    }
}
