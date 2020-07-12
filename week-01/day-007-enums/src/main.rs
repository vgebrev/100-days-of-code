enum IPAddressKind {
    V4,
    V6
}

// Enum and data in a struct
struct IPAddress {
    kind: IPAddressKind,
    address: String,
}

// Enum with data
enum IPAddress2 {
    V4(u8, u8, u8, u8),
    V6(String)
}

enum Message {
    Write(String),
    Ack,
}

impl Message {
    fn call(&self) {
        // TODO
    }
}

fn main() {
    let home1 = IPAddress {
        kind: IPAddressKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback2 = IPAddress {
        kind: IPAddressKind::V6,
        address: String::from("::1")
    };

    let home2 = IPAddress2::V4(120, 0, 0, 1);
    let loopback2 = IPAddress2::V6(String::from("::1"));

    let message = Message::Write(String::from("Hello, world!"));
    message.call();

    let some_number = Some(5);
    let some_string = Some("a string!");
    let absent_number: Option<u32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y; // Compiler error <- not handling Option::None
}
