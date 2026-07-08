use std::collections::HashMap;

fn main() {
    let num_list = vec![2, 3, 6, 3, 1, 5, 3, 9, 4, 6];

    let median = get_median(num_list.clone());

    println!("Median - {median}");

    let mode = get_mode(num_list);

    println!("Mode - {mode}");
}

fn get_median(mut list: Vec<i32>) -> f64 {
    list.sort();

    for e in &list {
        print!("{e}, ");
    }
    println!();

    let length = list.len();

    if length % 2 == 1 {
        // Odd
        list[length / 2] as f64
    } else {
        // Even
        let mid1 = list[length / 2 - 1];
        let mid2 = list[length / 2];
        (mid1 + mid2) as f64 / 2.0
    }
}

fn get_mode(list: Vec<i32>) -> i32 {
    let mut map_count = HashMap::new();

    let mut max_count = 0;
    let mut mode = 0;
    for e in &list {
        let count = map_count.entry(*e).or_insert(0);
        *count += 1;
        if *count > max_count {
            max_count = *count;
            mode = *e;
        }
    }

    mode
}
