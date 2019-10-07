#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    hobby: String
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

fn main() {
    let user = User{
        name: String::from("jean"),
        age: 22,
        hobby: String::from("basket")
    };
    println!("{:?}", user);
    println!("{:?}", user.age);
    println!("{:?}", user.name);
    let u1 = build(26, String::from("flores"), String::from("basket"));
    println!("{:?}", u1);
    let user2 = User{
        name: String::from("jean"),
        age: user.age,
        // hobby: user.hobby.clone()
        ..user
    };
    let rect1 = Rectangle{
        width: 30, 
        height: 50
    };
    let rect2 = Rectangle{
        width: 40, 
        height: 50
    };
    println!("The area is {}: ", rect1.area());
    println!("The area is {}: ", rect1.can_hold(&rect2));
    let rect3 = Rectangle::build(2, 3);
}

fn build(age: i32, name: String, hobby: String) -> User{
    User{
        age,
        name,
        hobby
    }
}

impl Rectangle{
    fn area(&self) -> i32 {
        self.width * self.height
    }      

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }    

    fn build(width: i32, height: i32) -> Rectangle {
        Rectangle{width, height}
    }
}