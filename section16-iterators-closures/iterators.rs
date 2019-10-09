fn main ()  {
    let mut v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter_mut();
    println!("{:?}", v1_iter);   
    assert_eq!(v1_iter.next(), Some(&mut 1));
    assert_eq!(v1_iter.next(), Some(&mut 2));
    assert_eq!(v1_iter.next(), Some(&mut 3));
    assert_eq!(v1_iter.next(), None);

    let v1: Vec<i32> = vec![1, 2, 3];
    let v2: Vec<_> = v1.iter().map(|x| x + 1 ).collect();
    assert_eq!(v2, vec![2,3,4]);
}

// trait Iterator  { 
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }