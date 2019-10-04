use std::io;

fn main() {
    q1();
}

fn q1() {
    let mut vowel = String::new();
    println!("Enter a vowel");
    io::stdin().read_line(&mut vowel).expect("Failed");
    // let vowel = vowel.trim().parse::<char>();
    let vowel: char = vowel.trim().parse().expect("Failed");
    if vowel == 'a' || vowel == 'e' || vowel == 'i' || vowel == 'o' || vowel == 'u'{
        println!("It's a vowel");
    } else { 
        println!("It isn't a vowel");
    }
}

fn q2() {
	let mut o=String::new();
	let a=10;
	let b=2;
	println!("Choose operation to be performed");
	println!("1. +\n2. -\n3. *\n4. /");
	io::stdin().read_line(&mut o).expect("Failed");
	//o=o.trim().to_string();
	
	if o=="+" {
		println!("{}",a+b);
	}else if o=="-" {
		println!("{}",a-b);
	}else if o=="*" {
		println!("{}",a*b);

	}else if o=="/" {
		println!("{}",a/b);
	
	}else {
		println!("Wrong Choice");
	}
}

fn q3() {
	let mut per=String::new();
	println!("Enter your percentage");
	io::stdin().read_line(&mut per).expect("Fail");
	let per:i32=per.trim().parse().expect("Fail");
	if per>=90 {
		println!("A Grade");
	}else if per>=80  {
		println!("B Grade")
	}else if per>=70{
		println!("C Grade");
	}else if per>=60 {
		println!("D Grade");
	}else {
		println!("Fail");
	
	}
}