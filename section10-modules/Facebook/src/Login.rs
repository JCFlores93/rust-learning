pub fn login(user: String, pass: String) -> i8 {
    if user == "Diwakar" && pass == "Diwakar" {
        println!("Logged in successfully");
        1
    }else {
        0
    }
} 