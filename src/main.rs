use std::{env, fs};
use rj::{parser, run};
use rj::method_area::MethodArea;


fn main() {
    let args: Vec<String> = env::args().collect();
    let class_path = args.get(1).expect("no main class passed in");
    run(class_path);
}
