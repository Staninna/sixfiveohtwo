// This is the testing file for the project (temporary)
use sixfiveohtwo::processor::Processor;
fn main() {
    let mut processor = Processor::new(0x00FF);
    println!("{:#?}", processor);
}
