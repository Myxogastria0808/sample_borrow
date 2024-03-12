use crate::sample_1::{Struct1, Struct2};

pub struct Struct3 {
    field_1: Struct1
}

impl Struct3 {
    pub fn new() -> Struct3 {
        Struct3 { 
            field_1: Struct1::new()
        }
    }

    pub fn get_field_1(&self) -> &Struct1 {
        &self.field_1
    }

    pub fn get_field_1_mut(&mut self) -> &mut Struct1 {
        &mut self.field_1
    }
}

pub struct Struct4<'a> {
    field_1: Struct2<'a>
}

impl<'a> Struct4<'a> {
    pub fn new(n_str: &str) -> Struct4 {
        Struct4 { 
            field_1: Struct2::new(n_str)
        }
    }

    pub fn get_field_1(&self) -> &Struct2 {
        &self.field_1
    }

    pub fn get_field_1_mut(&'a mut self) -> &mut Struct2 {
        &mut self.field_1
    }
}