// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
//     let four = IpAddrKind::V4;
//     let six = IpAddrKind::V6;

//     route(four);
//     route(six);

    // let home = IpAddr{
    //     kind: IpAddrKind::V4,
    //     address: String::from ("127.0.0.1"),
    // };

    // let loopback = IpAddr{
    //     kind: IpAddrKind::V6,
    //     address: String::from ("::1"),
    // };

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    route(home);
    route(loopback);
}

// fn route(ip_kind: IpAddrKind) {}
fn route(ip: IpAddr) {}
