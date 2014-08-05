// Currently, the `phase` feature must be enabled in order to import a crate
// that defines a syntax extension.
#![feature(phase)]
 
// The `phase` attribute is used here to indicate that the `factorial` crate
// provides a syntax extension.
// It's also possible for `factorial` to provide things other than a syntax
// extension, in which case, `#[phase(plugin, link)]` is required.
#[phase(plugin, link)] extern crate tyrant;

use tyrant::StringMatch;
use std::iter::Chain;

parse_string!(e "")
parse_string!(house "house")
parse_string!(cat "cat")
alt!(house cat)
opt!(house)
 
fn main() {
    let x: Vec<char> = "housecat".chars().collect();
    let y: Vec<char> = "catdog".chars().collect();
    
    for res in alt_house_cat(x.as_slice()) {
        println!("{}", res);
    }

    println!("--------");

    for res in alt_house_cat(y.as_slice()) {
        println!("{}", res);
    }

    println!("--------");

    for res in e(x.as_slice()) {
        println!("{}", res);
    }

    println!("--------");

    for res in e(y.as_slice()) {
        println!("{}", res);
    }

    println!("--------");

    for res in opt_house(x.as_slice()) {
        println!("{}", res);
    }
}
