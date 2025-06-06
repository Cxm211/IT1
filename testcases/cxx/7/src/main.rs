extern crate cxx;
use cxx::{let_cxx_string, CxxString};

fn takes_cxx_string(_s: &CxxString) {}

fn main() {
    let s = "s";
    let_cxx_string!(s = s);
    takes_cxx_string(&s);
}