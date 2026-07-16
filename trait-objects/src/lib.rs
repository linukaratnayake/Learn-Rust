pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Button struct and its implementation

/// A data type defining a button
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

/// Implementation of methods for data type `Button`
impl Button {
    pub fn print_button_details(&self) {
        println!("Just printing the details of the button...");
    }
}

/// Implementation of the `Draw`` trait for data type `Button`
impl Draw for Button {
    fn draw(&self) {
        println!(
            "Drawing a button... Width - {}, Height - {}",
            self.width, self.height
        );
    }
}

// TextField struct and its implementation

/// A data type defining a text field
pub struct TextField {
    content: String,
    length: usize,
}

/// Implementation of methods for data type "TextField"
impl TextField {
    pub fn new(text: String) -> Self {
        let length = text.len();
        TextField {
            content: text,
            length: length,
        }
    }
}

/// Implementation of the `Draw`` trait for data type `TextField`
impl Draw for TextField {
    fn draw(&self) {
        println!(
            "Drawing a text field... Content - {}, Length - {}",
            self.content, self.length
        );
    }
}
