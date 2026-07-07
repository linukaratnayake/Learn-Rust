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

    if let IPVariant::V6(ref ip) = other_ip {
        // This is the important part we want to run depending on whether it matches.
        println!("This is version 6. IP - {ip:?}");
    } else {
        // This part runs if it doesn't match.
        println!("This is version 4.");
    }

    let IPVariant::V4(f1, f2, _, _) = my_ip else {
        println!("It is not a IPv4 address.");
        return; // Stops executing.
    };

    // Now the fields f1, f2 are in the main scope. I can do whatever I want with them without going inside many curly braces.

    println!("The first two fields of the IPv4 address are {f1} and {f2}.");
}
