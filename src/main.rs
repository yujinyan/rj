use std::{env, fs};


fn main() {
    let args: Vec<String> = env::args().collect();
    let class_path = args.get(1).expect("no main class passed in");
    rj::run(class_path);
}
