use std::fmt::Display;
use std::io::Write;

use chrono::TimeZone;

mod a001_linked_list;
mod r001_to_string;
mod r002_mpmc;
mod r003_ceil_floor_round;


#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = 5;
    let b = &a;
    let c = &a;
    let d = &a;

    println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);
}
//     let mut a = Point { x: 0, y: 0 };
//     let b = &a;
//
//     a.x += 10;
//
//     println!("a: {:?}", a);
//     println!("b: {:?}", b);
// }