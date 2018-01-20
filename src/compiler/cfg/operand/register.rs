use std::fmt;

// For now registers will not have a type field because every register will
// be of type "Type*" which will be handled by the C lib.
#[derive(Debug, Clone)]
pub struct Register {
    pub label: String
}

impl Register {
    pub fn new() -> Register {
        unsafe {
            static mut SUFFIX: usize = 0;
            let label = format!("r{}", SUFFIX);
            SUFFIX += 1;

            Register { label }
        }
    }

    pub fn get_label(&self) -> String {
        self.label.clone()
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "%{}", self.label)
    }
}
