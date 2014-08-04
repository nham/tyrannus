#![feature(phase)]
#[phase(plugin)] extern crate step1;

parse_str!()

fn main() {
    println!("{}", foo(1));
}
