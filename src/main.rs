extern crate sample_for_book;
use sample_for_book::{Struct2, Struct3, Struct4};

fn main() {
    // let mut struct_3_1: Struct3 = Struct3::new();
    // struct_3_1.get_field_1_mut().set_field_1(5);
    // let u32_1: u32 = struct_3_1.get_field_1().get_field_1();
    // println!("{}", u32_1);
    let mut struct_4_1: Struct4 = Struct4::new("Sample");
    {
        let struct_2_1: &mut Struct2<'_> = struct_4_1.get_field_1_mut();
        struct_2_1.set_field_1("Change");
    }
    // struct_4_1.get_field_1_mut().set_field_1("Change");
    // let u32_2: &str = struct_4_1.get_field_1().get_field_1();
    // println!("{}", u32_2);
}