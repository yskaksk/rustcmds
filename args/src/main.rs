use std::env::args;

fn main() {
    let arguments = args();
    let mut count = 0;
    for arg in arguments {
        println!("{}", arg);
        count += 1;
    }
    println!("# of args: {}", count);
}
