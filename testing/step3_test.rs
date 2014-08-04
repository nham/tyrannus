#![feature(phase)]
#[phase(plugin)] extern crate step3;

parse_str!("horseradish")

fn main() {
    println!("{}", foo(6));
}
