#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// We can also put data inside an enum.
enum IpAddr {
    V4(String),
    V6(String),
}

enum IpAddr_v2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// This is how it looks in the standard library.
//
//  - <https://doc.rust-lang.org/std/net/enum.IpAddr.html>
//
// This shows that you can put any kind of data inside an enum variant: strings, numeric types, or structs.
//
// ```rust
// struct Ipv4Addr {
//     // --snip--
// }
//
// struct Ipv6Addr {
//     // --snip--
// }
//
// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }
// ```

// This is another example of an enum, which has four variants.
//
// This would be the equivalent of having four different structs:
//
// ```rust
// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct
// ```
//
// But if we used the different structs, we could not as easily define a function to take any of these kind of messages.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// There is another similarity between enums and structs: we can define methods on enums too.
impl Message {
    fn call(&self) {

    }
}

fn route(ip_kind: IpAddrKind) {
    println!("ip_kind = {ip_kind:?}");
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six);

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    let home = IpAddr_v2::V4(127, 0, 0, 1);
    let loopback = IpAddr_v2::V6(String::from("::1"));

    message_enum();
}

fn message_enum() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
