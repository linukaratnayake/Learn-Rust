#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn enlarge(&mut self, scale: u32) {
        self.width = self.width * scale;
        self.height = self.height * scale;
    }

    fn default_rectangle() -> Self {
        Self {
            width: 2,
            height: 1,
        }
    }
}

fn main() {
    let mut rect1 = Rectangle {
        width: dbg!(2 * 50),
        height: 20,
    };

    println!("The area of the rectangle is {}.", rect1.area());

    let scale = 3;
    println!("Enlarging the rectangle by a factor of {scale}...");
    rect1.enlarge(3);

    println!("The area of the new rectangle is {}.", rect1.area());

    println!("Printing rect1 {rect1:#?}");

    // Printing the rectangle with debug
    dbg!(&rect1);

    println!("\n\n\n");

    let default_rect = Rectangle::default_rectangle();

    dbg!(default_rect);
}
