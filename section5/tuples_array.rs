fn main() {
    let a:(i32, bool, f64) = (230, true , 8.5);
    println!("{:?}", a.0);
    print(a);
}

fn print(x:(i32, bool, f64)) {
    let (a, y, z) = x;
    println!("{} {} {}", a, y, z);
}

fn arrays() {
    let mut a: [i32; 5] = [23,32,23,23,23];
    a[4] = 1;
    println!("{:?}", a);
    print_array(a);
}
fn print_array(x:[i32 ;5]) {
    println!("{}", x.len());
    for n in x.iter() {
        println!("{}", n);
    }
}