// Imports
use crate::device::{Device, Ram};
use std::fmt;

// Structs

// Struct for the memory regions
struct Region {
    start: u16,
    end: u16,
    device: Box<dyn Device>,
}

// Implement the Region struct
impl Region {
    pub fn new(start: u16, end: u16, device: Box<dyn Device>) -> Self {
        Self { start, end, device }
    }

    pub fn contains(&self, address: u16) -> bool {
        address >= self.start && address <= self.end
    }
}

// Struct for the memory mapper
pub struct DeviceMapper {
    regions: Vec<Region>,
}

// Implement the DeviceMapper struct
impl DeviceMapper {
    pub fn new() -> Self {
        let ram = Box::new(Ram::new(0xFFFF));

        Self {
            regions: vec![Region::new(0x0000, 0xFFFF, ram)],
        }
    }

    pub fn map(&mut self, start: u16, end: u16, device: Box<dyn Device>) {
        // Create a new region
        let region = Region::new(start, end, device);

        // add the region to the regions vector at the first position
        self.regions.insert(0, region);
    }

    pub fn unmap(&mut self, start: u16, end: u16) {
        // Find the index of the region with the given start and end addresses
        let index = self
            .regions
            .iter()
            .position(|r| r.start == start && r.end == end)
            .unwrap();

        // Remove the region from the regions vector
        self.regions.remove(index);
    }

    pub fn read(&self, address: u16) -> u8 {
        // Find the region that contains the given address
        let region = self.regions.iter().find(|r| r.contains(address)).unwrap();

        // Read the data from the device in the region
        let offset = address - region.start;
        region.device.read(offset)
    }

    pub fn write(&mut self, address: u16, data: u8) {
        // Find the region that contains the given address
        let region = self
            .regions
            .iter_mut()
            .find(|r| r.contains(address))
            .unwrap();

        // Write the data to the device in the region
        let offset = address - region.start;
        region.device.write(offset, data);
    }
}

// Implement the Debug trait for the DeviceMapper struct
impl fmt::Debug for DeviceMapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceMapper {{")?;
        for region in &self.regions {
            write!(
                f,
                " {:04X}-{:04X}: {},",
                region.start,
                region.end,
                region.device.get_type()
            )?;
        }
        write!(f, "}}")
    }
}
