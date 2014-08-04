#![feature(phase)]
#[phase(plugin)] extern crate step2;

parse_str!()

fn main() {
    println!("{}", foo(1));
}
