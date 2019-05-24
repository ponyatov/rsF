use std::env;
use std::io;

fn main() {
	
	let mut pad = String::new();

    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
    
    io::stdin()
	    .read_line(&mut pad)
	    .expect("error");
    println!("{}",pad);
    
}
