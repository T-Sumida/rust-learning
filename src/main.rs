enum IpAddrKind {
    V4,
    V6,
}


struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Option<T> {
    Some(T),
    None,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    {
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;
    }
    
    {
        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };
        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
    }

    {
        let some_number = Some(5);
        let some_string = Some("a string");

        // let absent_number: Option<i32> = None;

        let x: i8 = 5;
        // let y: Option<i8> = Some(5);

        // let sum = x + y;
    }
    {
        let value = value_in_cents(Coin::Penny);
        println!("{}", value);
    }
    
    {
        let five = Some(5);
        let six = plus_one(five);
        // let none = plus_one(None);
    }
}