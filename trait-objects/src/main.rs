use trait_objects::{Button, Draw, Screen, TextField};

// Suppose the user of the library wants to define his own data types

struct CheckBox {
    ticked: bool,
    label: String,
}

impl CheckBox {
    pub fn print_checkbox_details() {
        println!("Just pring the details of the checkbox.");
    }
}

impl Draw for CheckBox {
    fn draw(&self) {
        println!(
            "Drawing a checkbox that is ticked: {}, Label - {}",
            self.ticked, self.label
        );
    }
}

// Main function that defines the instances, create a `Screen` and `draw`.
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 5,
                height: 3,
                label: "Click This".to_string(),
            }),
            Box::new(CheckBox {
                ticked: true,
                label: "I agree.".to_string(),
            }),
            Box::new(TextField::new(String::from("Hello World"))),
        ],
    };

    screen.run();
}
