extern crate Facebook;
use Facebook::*;

fn main() {
    println!("Hello, world!");
    let user = String::from("Dikawar");
    let pass = String::from("Dikawar");
    let s = Login::login(user, pass);
    if s == 1 {
        Post::post(String::from("Hello how are you"));
        Logout::logout();
    } else { 
        println!("Invalid");
    }
}
