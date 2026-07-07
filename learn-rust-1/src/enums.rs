enum IPVariant {
    V4(u8, u8, u8, u8),
    V6(String),
}

impl IPVariant {
    fn print(&self) {
        match self {
            IPVariant::V4(field1, field2, field3, field4) => {
                println!("{field1}.{field2}.{field3}.{field4}")
            }
            IPVariant::V6(s) => {
                println!("{s}")
            }
        }
    }
}

fn main() {
    let my_ip = IPVariant::V4(192, 168, 0, 1);

    let other_ip = IPVariant::V6("::1".to_string());

    my_ip.print();

    other_ip.print();
}
