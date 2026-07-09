use generics::auxiliary_functions;
fn main() {
    let nunmber_list = vec![2, 5, 6, 3, 9, 1, 4];

    let largest = auxiliary_functions::largest_element(&nunmber_list);

    println!("The largest number is {largest}");

    let char_list = vec!['s', 'a', 'f', 'h', 'u'];

    let largest = auxiliary_functions::largest_element(&char_list);

    println!("The largest number is {largest}");
}
