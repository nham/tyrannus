#![feature(phase)]
#[phase(plugin)] extern crate step4;

parse_str!(foo "horseradish")
parse_str!(bar "housecat")

fn main() {
    println!("{}", foo(6));
    println!("{}", bar(4));
}
