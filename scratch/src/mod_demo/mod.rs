pub mod mod_contents;

pub fn mod_demo_root() -> u32 {
    42
}

pub struct ModDemo {
    pub a: u32,
    b: String,
    c: f32,
}

impl ModDemo {
    pub fn new(a: u32, b: String, c: f32) -> ModDemo {
        ModDemo { a, b, c }
    }

    pub fn add(&self) -> f32 {
        self.c + (self.a as f32)
    }

    pub fn set_c(&mut self, new_c: f32) {
        self.c = new_c;
        self.modify_string()
    }

    fn modify_string(&mut self) {
        self.b += "!";
    }

    pub fn see_string(&self) -> &str {
        &self.b
    }
}

pub struct PrivModTupleStruct(u32);
pub struct PubModTupleStruct(pub u32);
