/// Enumerations are Sum types in Abstract Data Types in Rust
pub fn call1() {
    enum IpAddrKind {
        V4,
        V6,
    }
    struct IpAddr1 {
        kind:    IpAddrKind,
        address: String,
    }

    #[derive(Debug)]
    enum IpAddr2 {
        V4(String),
        V6(String),
    }

    #[derive(Debug)]
    enum IpAddr3 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    #[derive(Debug)]
    enum IpAddr4 {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    #[derive(Debug)]
    struct Ipv4Addr {
        address: String,
    }
    #[derive(Debug)]
    struct Ipv6Addr {
        address: String,
    }

    let four = IpAddrKind::V4;

    let six = IpAddrKind::V6;

    let home1 = IpAddr1 { kind:    IpAddrKind::V4,
                          address: String::from("127.0.0.1"), };

    let loopback1 = IpAddr1 { kind:    IpAddrKind::V6,
                              address: String::from("::1"), };

    println!("home address: {}", home1.address);
    println!("loopback address: {}", loopback1.address);

    let home2 = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback2 = IpAddr2::V6(String::from("::1"));

    println!("home address: {home2:#?}");
    println!("loopback address: {loopback2:#?}");

    let home3 = IpAddr3::V4(127, 0, 0, 1);
    let loopback3 = IpAddr3::V6(String::from("::1"));

    println!("home address: {home3:#?}");
    println!("loopback address: {loopback3:#?}");

    let home_v4 = Ipv4Addr { address: String::from("127.0.0.1"), };

    let loopback_v6 = Ipv6Addr { address: String::from("::1"), };

    let home4 = IpAddr4::V4(home_v4);
    let loopback4 = IpAddr4::V6(loopback_v6);

    println!("home address: {home4:#?}");
    println!("loopback address: {loopback4:#?}");
}

pub fn call2() {
    enum Message {
        Quit,
        Move { x: i32, y: i32, },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // Unit Struct
    //
    // -- in Haskell
    // data QuitMessage = QuitMessage
    struct QuitMessage;

    struct MoveMessage {
        x: i32,
        y: i32,
    }

    // Tuple Struct
    //
    // -- in Haskell
    // data WriteMessage = WriteMessage String
    struct WriterMessage(String);

    // -- in Haskell
    // data ChangeColorMessage = ChangeColorMessage Int Int Int
    struct ChangeColorMessage(i32, i32, i32);
}
