// This is the testing file for the project (temporary)
#![allow(
    dead_code,
    unused_variables,
    unused_imports,
    unused_mut,
    unused_assignments
)]

use sixfiveohtwo::processor::Processor;
fn main() {
    let mut processor = Processor::new(0x00FF);
    println!("{:#?}", processor);
}
