extern crate sample_for_book;
#[allow(unused_imports)]
use sample_for_book::{Struct1, Struct2};

fn main() {
    let mut struct_1_1: Struct1 = Struct1::new();
    struct_1_1.set_field_1(5);
    println!("{}", struct_1_1.get_field_1());
    let mut struct_1_1: Struct2 = Struct2::new();
    struct_1_1.set_field_1(&5);
    // エエラー発生
    // println!("{}", struct_1_1.get_field_1());
}