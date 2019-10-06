
fn main() {
    add();
    add_params(2, 4);
    let sum = add_params_return_type(1, 2);
    println!("{}", sum);
    println!("{:?}", add_params_return_multiple_values(5, 3));
    nested_functions();
    println!("{}", factorial(5));
}

fn add() {
    println!("{}", 2 + 3)
}

fn add_params(a:i32, b:i32) {
    println!("{}", a + b)
}

fn add_params_return_type(a:i32, b:i32) -> i32 {
    return a + b
}

fn add_params_return_multiple_values(a:i32, b:i32) -> (i32, i32) {
    (a + b, a - b)
}

fn nested_functions(){
    add();
    fn add() {
        println!("add");
    }
}

fn factorial(a: i32) -> i32 {
    let mut initial = 1;
    let mut a = a;
    while a != 0 {
        initial *= a;
        a -= 1;
    }
    return initial;
}
