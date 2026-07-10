use tests::greeting;

#[test]
fn greeting_contains_name() {
    let name = String::from("John");
    let result = greeting(&name);
    assert!(result.contains(&name));
}

#[test]
fn parse_integer_should_go_wrong() {
    let number_string = "53a";
    let result = number_string.parse::<i32>();

    assert!(result.is_err())
}
