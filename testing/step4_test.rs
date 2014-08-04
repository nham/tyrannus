#![feature(phase)]
#[phase(plugin)] extern crate step4;

parse_str!(foo "horseradish")

fn main() {
    println!("{}", foo(6));
}
