
fn main() { 
    filters_by_size();
    using_other_iterator_trait_methods()
}

#[derive(PartialEq, Debug)]
struct Shoe { 
    size: u32,
    style: String
}

struct Counter { 
    count: u32,
}

impl Iterator for Counter { 
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 { 
            Some(self.count)
        }else { 
            None
        }
    }
}

impl Counter {
    fn new() -> Counter {
        Counter{ count: 0}
    }
}

fn using_other_iterator_trait_methods() { 
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
    .map(|(a, b)| a * b)
    .filter(|x| x % 3 == 0)
    .sum();
}

fn shoe_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn filters_by_size() {
    let shoe = vec![
    Shoe{size: 10, style: String::from("sneaker")},
    Shoe{size: 13, style: String::from("sandal")},
    Shoe{size: 10, style: String::from("boot")}
    ];
    let in_my_size = shoe_in_my_size(shoe, 10);
    assert_eq!(in_my_size, vec![ 
    Shoe{size: 10, style: String::from("sneaker")},
    Shoe{size: 10, style: String::from("boot")}
    ]);
}