mod games;
mod tests;
mod common_concepts;

fn main() {
    let square = Rectangle::square(3);
    println!("The area of the square is: {}", square.area());

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("rect1 is {:#?}", rect1);
    println!("rect2 is {:#?}", rect2);
}

#[derive(Debug)]
struct UserInformation {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: String, email: String) -> UserInformation {
    UserInformation {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

#[derive(Debug)]
struct AlwaysValid;

const MY_TYPE: AlwaysValid = AlwaysValid;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    fn square(size: u32) -> Self {
        return Self {
            width: size.clone(),
            height: size,
        };
    }
}