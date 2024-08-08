#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    // let guess: u32 = "42".parse().expect("not a number");
    // let x: f64 = 2.0;
    // let x: f32 = 2.0;
    // addition
    // let sum: i32 = 5 + 10;

    // // subtraction
    // let difference: f64 = 95.5 - 4.3;

    // // multiplication
    // let product: i32 = 4 * 30;

    // // division
    // let quotient: f64 = 56.7 / 32.2;
    // let truncated: i32 = -5 / 3; // 结果为 -1

    // // remainder
    // let remainder: i32 = 43 % 5;

    // let t: bool = false;

    // let c: char = 'z';
    // let z: char = 'ℤ'; // with explicit type annotation
    // let heart_eyed_cat: char = '😻';
    // let tup: (i32, f64, i32) = (500, 6.4, 1);

    // let (_x, y, z) = tup;

    // println!("The value of y is: {y}");
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let condition = true;
    // let x = if condition { 5 } else { 6 };

    // struct User {
    //     active: bool,
    //     username: String,
    //     email: String,
    //     sign_in_count: u64,
    // }

    // let mut user1 = User {
    //     active: true,
    //     username: String::from("user"),
    //     email: "1@qq.com".to_owned(),
    //     sign_in_count: 1,
    // };
    // user1.email = "2@qq.com".to_owned();

    // let user2 = User{
    //     email: String::from("3@dd.com"),
    //     ..user1
    // };

    // fn build_user(email: String, username: String) -> User {
    //     User {
    //         active: true,
    //         username,
    //         email,
    //         sign_in_count: 1,
    //     }
    // }

    // struct Color(i32,i32,i32);
    // struct Point(i32,i32,i32);
    // let black = Color(0,0,0);

    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2: Rectangle = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3: Rectangle = Rectangle {
        width: 60,
        height: 45,
    };

    // println!("area = {}", rect1.area())
    // if rect1.width() {
    //     println!("The rectangle has a nonzero width; it is {}", rect1.width);
    // }
    println!("can_hold {}", rect1.can_hold(&rect2))
}

// fn area(rect: &Rectangle) -> u32 {
//     rect.width * rect.height
// }
