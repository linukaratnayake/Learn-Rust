fn main() {
    let condition = true;

    let tup = if condition {
        (4, "Hello")
    } else {
        (1, "There")
    };

    my_custom_function(tup);
}

fn my_custom_function(tuple: (i32, &str)) {
    let (x, y) = tuple;
    println!("the value is {x}, {y}");
}
