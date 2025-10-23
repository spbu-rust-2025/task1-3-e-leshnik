use std::fs;
use std::io;

fn main() {
    let mut path = String::new();
    
    if io::stdin().read_line(&mut path).is_ok() {
        if fs::read(path.trim()).is_ok() {
            println!("success");
        } else {
            println!("failure");
        }
    } else {
        println!("failure");
    }
}
