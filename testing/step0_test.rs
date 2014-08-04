static foo_str: &'static str = "abc";

fn foo(n: uint) -> (&'static str, &'static str) {
    (foo_str.slice_to(n), foo_str.slice_from(n))
}

fn main() {
    println!("{}", foo(1));
}
