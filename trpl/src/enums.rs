// pub enum IpAddrKind {
//     V4,
//     V6,
// }

// pub struct IpAddr  {
//     kind: IpAddrKind,
//     address: String,
// }

pub enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

pub fn route(ip_kind: IpAddr) {
    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1"),
    // };
    let home = IpAddr::V4(127, 0, 0, 1);
    let m = Message::Write(String::from("Write"));
    m.call();

    let some_number: Option<u32> = Some(4);
    let n =  some_number.unwrap_or(1);
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state)=>{
            println!("{:?}", state);
            25
        },
    }
}