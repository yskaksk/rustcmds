use std::env::args;

fn main() {
    let arguments: Vec<String> = args().collect();
    println!("argc={}", arguments.len());
    let mut count = 0;
    for arg in arguments {
        println!("argv[{}]={}", count, arg);
        count += 1;
    }
}
