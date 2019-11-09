// https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html

use std::env;

fn main(){
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}