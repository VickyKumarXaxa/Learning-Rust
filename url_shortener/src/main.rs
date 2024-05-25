use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Incorrect number of arguments");
    } else {
        println!("Got correct number of arguments");
        let arg1 = &args[1];
    }
}
