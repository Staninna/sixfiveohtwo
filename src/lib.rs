// Public modules
pub mod device;
pub mod opcodes;
pub mod processor;

// Private modules
mod device_mapper;
mod registers;

// Unit tests
#[cfg(test)]
mod test_opcodes;
