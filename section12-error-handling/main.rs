use std::fs::File;
use std::io::Read;
use std::io;
use std::io::ErrorKind;

fn main() {
	let f=File::open("hello.txt");
	let f= match f {
		Ok(file)=>file,
		Err(ref error) if error.kind()==ErrorKind::NotFound=> {
			match File::create("hello.txt") {
				Ok(fc)=>fc,
				Err(e)=> {
					panic!("Not able to create file {:?}",e)
				},
			}
		},
		Err(error)=> { panic!("Unable to Open File{:?}",error); },
	};

	let g = File::open("hello.txt").unwrap();
	let g = File::open("hello.txt").expect("Failed");	

	let output = read();
	match output {
		Ok(fi) => println!("{:?}", fi),
		Err(e) => println!("{:?}", e)
	}
}

// fn read() -> Result<String, io::Error> {
// 	let f=File::open("hello.txt");
// 	let mut f= match f {
// 		Ok(file) => file,
// 		Err(e) => return Err(e),
// 	};

// 	let mut s = String::new();
// 	match f.read_to_string(&mut s) {
// 		Ok(_) => Ok(s),
// 		Err(e) => Err(e),
// 	}
// }

fn read() -> Result<String, io::Error> {
	let mut f=File::open("hello.txt")?;

	let mut s = String::new();
	f.read_to_string(&mut s)?;
	Ok(s)
}