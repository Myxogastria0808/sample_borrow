pub struct Struct1 {
    field_1: u32
}

impl Struct1 {
    pub fn new() -> Struct1 {
        Struct1 { 
            field_1: 0 
        }
    }

    pub fn get_field_1(&self) -> u32 {
        self.field_1
    }

    pub fn set_field_1(&mut self, sf1_u32: u32) {
        self.field_1 = sf1_u32
    }
}

pub struct Struct2<'a> {
    field_1: &'a str
}

impl<'a> Struct2<'a> {
    pub fn new(n_str: &str) -> Struct2 {
        Struct2 { 
            field_1: n_str
        }
    }

    pub fn get_field_1(&self) -> &str {
        self.field_1
    }

    pub fn set_field_1(&'a mut self, sf1_str: &'a str) {
        self.field_1 = sf1_str;
    }
}

impl Drop for Struct2<'_> {
    fn drop(&mut self) {
        
    }
}