#[derive(Debug)]
enum SpreadSheet {
    Integer(i32),
    Float(f64),
    Text(String)
}

use std::collections::HashMap;

fn main() {
    let mut v = Vec::new();
    v.push(20);
    v.push(21);
    v.push(22);
    v.push(23);
    println!("{:?}", v);

    let mut y:Vec<i32> = vec![1, 2, 3, 4];
    println!("{:?}", y);

    let x = y.get(0);
    println!("{:?}", x);

    for i in &mut v {
        *i *= 2;
        println!("{}", i);
    }

    println!("{:?}", v);

    let row = vec![SpreadSheet::Integer(3), 
    SpreadSheet::Float(3.4), 
    SpreadSheet::Text(String::from("Hello"))];
    println!("{:?}", row);

    let a = 1;
    let mut s = a.to_string();
    println!("{:?}", s);
    s.push_str("Hello");
    println!("{:?}", s);

    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = s1 + &s2;
    println!("{:?}", s3);

    let sb1 = String::from("Hello");
    let sb2 = String::from("World");
    let sb3 = format!("{} {}",sb1, sb2);
    println!("{:?}", sb3);
    println!("{:?}", sb1);
    println!("{:?}", sb2);

    let sbc1 = String::from("Hello");
    let sbc2 = &sbc1[0..2];
    println!("{:?}", sbc1);
    println!("{:?}", sbc2);

    let mut score = HashMap::new();
    score.insert("Blue", 10);
    score.insert("Red", 20);
    println!("{:?}", score);

    let team = vec!["Blue", "Red"];
    let score = vec![10, 20];
    let scores:HashMap<_,_> = team.iter().zip(score.iter()).collect();
    println!("{:?}", scores);   
    let mut scores1 = HashMap::new();
    scores1.insert("Blue", 10);
    scores1.insert("Yellow", 10);

    println!("{:?}", scores1.get("Blue"));  

    // Updating a map
    let mut scorehm = HashMap::new();
    scorehm.insert("Blue",10);
    scorehm.entry("Blue").or_insert(20);
    scorehm.entry("Red").or_insert(30);
    println!("{:?}", scorehm);  

}