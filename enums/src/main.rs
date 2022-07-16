#[derive(Debug)]
enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Self is {:?}", self)
    }
}

fn main() {
    let four = IpAddrKind::V4(127,0,0,1);
    let six = IpAddrKind::V6(String::from("::1"));
    route(four);

    let m = Message::Write(String::from("hello"));
    let b = Message::Move{x:24, y:14};
    m.call();
    b.call();

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = match y {
        None => None,
        Some(value) => Some(value + x),
    };



    if let Some(value) = sum { println!("Sum is {:?}", sum); }
}

fn route(ip_kind: IpAddrKind) {
    println!("Route is {:?}", ip_kind)
}
