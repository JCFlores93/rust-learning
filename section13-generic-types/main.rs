// Generic Types, Traits, Lifetimes
fn main() {
    let list = vec![23, 4, 64, 65];
    let result = largest(&list);
    let mut largest = list[0];
    for n in list { 
        if n > largest { 
            largest = n;
        }
    }
}

// fn largest (list: &[i32]) -> i32 {
//     let mut largest = list[0];
//      for &n in list { 
//         if n > largest { 
//             largest = n;
//         }
//     }
//     largest
// }

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
     for &n in list { 
        if n > largest { 
            largest = n;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];
     for &n in list { 
        if n > largest { 
            largest = n;
        }
    }
    largest
}