use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() { 
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("Hello from spawn thread {}", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //         println!("Hello from main thread {}", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // handle.join().unwrap();

    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(move || {
    //     println!("{:?}", v);
    // });
    // handle.join().unwrap();

    let (s, r) = mpsc::channel();
    let s1 = mpsc::Sender::clone(&s);
    thread::spawn(move || {
        // let val = String::from("hi");
        let vals = vec!["hi", "from", "the", "thread"];
        for val in vals { 
            s1.send(val).unwrap();
            println!("val: {}", val);
            thread::sleep(Duration::from_secs(1));
        }        
    });
    thread::spawn(move || {
        // let val = String::from("hi");
        let vals = vec!["more", "messages", "for", "you"];
        for val in vals { 
            s.send(val).unwrap();
            println!("val: {}", val);
            thread::sleep(Duration::from_secs(1));
        }        
    });

    for received in r { 
        // let rec = r.recv().unwrap();
        println!("Got {}", received);
    }

    let (s2, r2)=mpsc::channel();
	let handle=thread::spawn(||{
		run(s2);
		run1(r2);
	});
	handle.join().unwrap();
    
}


fn run(s: mpsc::Sender<i32>) {
	s.send(2).unwrap();
	s.send(3).unwrap();

}
fn run1(r:mpsc::Receiver<i32>) {
	let rec=r.recv().unwrap();
	println!("{}",rec);
	println!("{}",r.recv().unwrap());

}