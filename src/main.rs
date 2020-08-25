#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}


fn main() {
    {
        let email =  String::from("someone@example.com");
        let username = String::from("someusername123");
        let user = build_user(email, username);
        let mut user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        
        let user2 = User {
            email: String::from("another@example.com"),
            username: String::from("anotherusername567"),
            ..user1
        };
    }

    // {
    //     let width1 = 30;
    //     let height1 = 50;

    //     println!(
    //         // 四角形の面積は、{}平方ピクセルです
    //         "The area of the rectangle is {} square pixels.",
    //         area(width1, height1)
    //     );
    // }

    // {
    //     let rect1 = (30, 50);
    //     println!(
    //         "The area of the rectangle is {} square pixels.",
    //         area(rect1)
    //     );
    // }
    {
        let rect1 = Rectangle {width: 30, height: 50};
        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect1)
        );
    }

    {
        let rect1 = Rectangle { width: 30, height: 50 };

        println!("rect1 is {:?}", rect1);
    }

    {
        let rect1 = Rectangle { width: 30, height: 50 };

        println!(
            "The area of the rectangle is {} square pixels.",
            rect1.area()
        );
    }

    {
        let rect1 = Rectangle { width: 30, height: 50 };
        let rect2 = Rectangle { width: 10, height: 40 };
        let rect3 = Rectangle { width: 60, height: 45 };

        // rect1にrect2ははまり込む？
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    }

    {
        let sq = Rectangle::square(3);
        println!("rect1 is {:?}", sq);
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}