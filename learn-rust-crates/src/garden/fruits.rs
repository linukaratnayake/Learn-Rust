#[derive(Debug)]
pub enum Fruits {
    APPLE,
    ORANGE,
    BANANA,
    GRAPE,
}

impl Fruits {
    pub fn print_name(&self) {
        println!("The fruit is {:?}", self);
    }
}
