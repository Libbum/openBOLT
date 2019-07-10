extern crate fit;

use std::path::PathBuf;
use fit::MessageType;

fn main() {
    let filepath = PathBuf::from("input/2019-07-03-070222-ELEMNT BOLT 70F7-51-0.fit");
    let msgs = fit::run(&filepath);
    for r in msgs.iter().filter(|m| m.kind == MessageType::Record).take(5) {
        for v in &r.values {
            match v.field_num {
                253 => println!("Timestamp, {:?}", v.value),
                0 => println!("Latitude, {:?}", v.value),
                1 => println!("Longitude, {:?}", v.value),
                2 => println!("Altitude, {:?} m", v.value),
                5 => println!("Distance, {:?} m", v.value), //TODO: /100
                6 => println!("Speed, {:?} m/s", v.value), //TODO: /1000)*3.6 -> km/h
                9 => println!("Grade, {:?} %", v.value),
                13 => println!("Temperature, {:?} C", v.value),
                31 => println!("GPS Accuracy, {:?} m", v.value),
                _ => println!("{}, {:?}", v.field_num, v.value)
            }
        }
    }
}
