extern crate fit;

use std::path::PathBuf;
use fit::MessageType;

fn main() {
    let filepath = PathBuf::from("input/2019-07-03-070222-ELEMNT BOLT 70F7-51-0.fit");
    let msgs = fit::run(&filepath);
    for r in msgs.iter().filter(|m| m.kind == MessageType::Record) {
        println!("{:?}", r.values);
    }
}
