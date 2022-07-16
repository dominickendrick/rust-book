#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}   

fn main() {
    let user1 = build_user( String::from("someone@example.com")
        ,String::from("someusername123"));

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    println!("{:#?}",user2);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let scale = 2;

    let rect1 = Rectangle {
        width: dbg!(30 * scale), 
        height: 50
    };

    dbg!(
        rect1,
        "The area of the rectangle is {} square pixels.",
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}


