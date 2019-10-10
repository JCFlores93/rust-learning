use List::{Cons,Nil};
fn main() {
	let list=Cons(1,
	Box::new(Cons(2,Box::new(Cons(3,Box::new(Nil))))));
}

enum List{
	Cons(i32,Box<List>),
	Nil,
}


use std::ops::Deref;

struct MyBox<T>(T);

impl <T> MyBox<T> {
	fn new(x:T)->MyBox<T> {
		MyBox(x)
	}
}

impl <T> Deref for MyBox<T> {
	type Target=T;
	
	fn deref(&self)->&T {
		&self.0
	}
}

fn main(){
	let x=5;
	let y= MyBox::new(5);
	assert_eq!(5,x);
	assert_eq!(5,*y);
}

enum List{
	Cons(i32,Rc<List>),
	Nil,
}
use List::{Cons,Nil};
use std::rc::Rc;
fn main() {
	let a=Rc::new(Cons(5,Rc::new(Cons(10,Rc::new(Nil)))));
	println!("Counter after creating a {} ",Rc::strong_count(&a));
	let b =Cons(3,Rc::clone(&a));
	println!("Counter after creating b {} ",Rc::strong_count(&a));
	{
		let c=Cons(4,Rc::clone(&a));
		println!("Counter after creating c {} ",Rc::strong_count(&a));
	}
	println!("Counter after dropping c {} ",Rc::strong_count(&a));
}

use std::cell::RefCell;

pub trait Messenger {
	fn send(&self,msg:&str);
}

pub struct LimitTracker <'a,T: 'a+Messenger> {
	messenger:&'a T,
	value:usize,
	max:usize,
}

impl <'a,T> LimitTracker <'a,T> 
	where T:Messenger {
		pub fn new (messenger:&T,max:usize)->LimitTracker<T> {
			LimitTracker {
				messenger,
				value:0,
				max,
			}
		}
		pub fn set_value(&mut self,value:usize) {
			self.value=value;
			let percentage_of_max= self.value as f64 / self.max as f64;
			
			if percentage_of_max >=0.75 && percentage_of_max <0.9 {
				self.messenger.send("Warning You used up over 75%");
			}else if percentage_of_max>=0.9 && percentage_of_max <1.0{
				self.messenger.send("Warning You used up over 90%");
			}else if percentage_of_max>=1.0 {
				self.messenger.send("You are over of your quota");
			}
			
			
			
			
		}
		
	}

#[cfg(test)]
mod test {
	use super::*;
	
	struct MockMessenger {
		sent_messages:RefCell<Vec<String>>,
		
		
	}
	impl MockMessenger {
		fn new() -> MockMessenger {
			MockMessenger {sent_messages:RefCell::new(vec![])}
		
		}
	
	}
	impl Messenger for MockMessenger {
		fn send(& self,message:&str){
			self.sent_messages.borrow_mut().push(String::from(message));
		}
	}
	
	
	#[test]
	fn it_sends_an_over_75_percent_warning_message() {
	let mock_messenger=MockMessenger::new();
	let mut limit_tracker=LimitTracker::new(&mock_messenger,100);
	limit_tracker.set_value(80);
	
	assert_eq!(mock_messenger.sent_messages.borrow().len(),1);
	
	
	
	}