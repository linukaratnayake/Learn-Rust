use generics::auxiliary_functions::{self, Point};
fn main() {
    let nunmber_list = vec![2, 5, 6, 3, 9, 1, 4];

    let largest = auxiliary_functions::largest_element(&nunmber_list);

    println!("The largest number is {largest}");

    let char_list = vec!['s', 'a', 'f', 'h', 'u'];

    let largest = auxiliary_functions::largest_element(&char_list);

    println!("The largest number is {largest}");

    let point1 = Point(3, 5);
    let point2 = Point(3.5, 4.5);

    // let distance1 = point1.distance_from_origin();
    let distance2 = point2.distance_from_origin();

    println!("The distance - {distance2}");
}
