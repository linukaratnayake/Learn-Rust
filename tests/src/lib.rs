fn add_two(item: i32) -> i32 {
    item + 2
}

fn greeting(name: &str) -> String {
    format!("Hello {name}, how are you?")
}

// A function that panics if the input is not within the range 1 - 100.
fn filter_range(input: i32) -> bool {
    if input < 1 || input > 100 {
        panic!("Out of the expected range.");
    }

    true
}

#[cfg(test)]
mod tests {
    use std::{fmt::Error, num::ParseIntError, result};

    use super::*;

    #[test]
    fn test_working() {
        let result = add_two(5);
        assert_eq!(7, result);
    }

    // #[test]
    // fn antoher_test() {
    //     panic!("Going away!"); // This test will always fail.
    // }

    #[test]
    fn greeting_contains_name() {
        let name = String::from("John");
        let result = greeting(&name);
        assert!(result.contains(&name));
    }

    #[test]
    #[should_panic]
    fn another_test_2() {
        let num1 = 2;
        let num2 = 3;
        assert_eq!(num1, num2, "You know {num1} and {num2} are not equal.");
    }

    #[test]
    #[should_panic]
    fn wrong_range() {
        filter_range(200);
    }

    #[test]
    #[should_panic(expected = "expected range")]
    fn wrong_range_2() {
        filter_range(200);
    }

    #[test]
    fn parse_integer() -> Result<(), ParseIntError> {
        let number_string = "53";
        let _: i32 = number_string.parse()?;
        Ok(())
    }

    #[test]
    fn parse_integer_should_go_wrong() {
        let number_string = "53a";
        let result = number_string.parse::<i32>();

        assert!(result.is_err())
    }
}
