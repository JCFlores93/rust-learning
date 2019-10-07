#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

#[derive(Debug)]
enum IpAddrKind1 {
    V4(String),
    // V4(u8, u8, u8, u8),
    V6(String)
}

#[derive(Debug)]
enum Fruits {
    Apple = 0,
    Mango = 1,
    Watermelon = 20
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?}", four);
    println!("{:?}", six);
    let home = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };
    let loopback = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };
    println!("{:?}", home);
    let f = Fruits::Mango;
    println!("{:?}", f)
}