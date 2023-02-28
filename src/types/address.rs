use std::ops::Add;

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Address([u8; 20]);

impl Address {
    pub fn to_string(&self) -> String {
        hex::encode(self.0)
    }

    pub fn from_bytes(b: &[u8]) -> Result<Self, String> {
        println!("conv part");
        if b.len() != 20 {
            println!("wrong len");
            return Err(format!("given bytes with length {} should be 20", b.len()));
        }

        let mut value = [0u8; 20];
        value.copy_from_slice(b);

        Ok(Address(value))
    }
    
}