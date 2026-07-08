use crate::garden::{fruits::Fruits, vegetables::Asparagus};

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");

    let fruits_vec = vec![Fruits::APPLE, Fruits::BANANA, Fruits::ORANGE];

    for fruit in &fruits_vec {
        println!("Printing... {fruit:?}");
    }

    let Some(my_fruit) = fruits_vec.get(2) else {
        println!("Out of bound.");
        return;
    };

    println!("My fruit - {my_fruit:?}");
}
