pub trait Device {
    fn get_type(&self) -> String;
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, data: u8);
}

// Ram device
pub struct Ram {
    data: Vec<u8>,
}

impl Ram {
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }
}

impl Device for Ram {
    fn get_type(&self) -> String {
        "Ram".to_string()
    }

    fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    fn write(&mut self, address: u16, data: u8) {
        self.data[address as usize] = data;
    }
}

// Stdout device
pub struct Stdout;

impl Device for Stdout {
    fn get_type(&self) -> String {
        "Stdout".to_string()
    }

    fn read(&self, _address: u16) -> u8 {
        0
    }

    fn write(&mut self, _address: u16, data: u8) {
        print!("{}", data as char);
    }
}
