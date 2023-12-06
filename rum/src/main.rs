use std::env;
use rum::exec;
use rum::rumload;
fn main() {
    let input = env::args().nth(1);
    let instructions = rumload::load(input.as_deref());
    let memory = exec::UM::new(instructions); 
    exec::execute(memory);
}