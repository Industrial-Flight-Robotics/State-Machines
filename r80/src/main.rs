
mod r80;
use r80::R80;

fn main() {

    println!("8080 / Z80 Emulator");
    println!("-------------------");

    let cpu = R80::new();

    cpu.dump();
}