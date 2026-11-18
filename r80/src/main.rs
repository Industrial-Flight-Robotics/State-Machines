
mod r80;
mod memory;

use r80::R80;
use memory::Memory;


fn main() {

    println!("8080 / Z80 Emulator");
    println!("-------------------");

    let cpu = R80::new();

    let mut memory = Memory::new();

    // Write values into memory

    memory.write(0x1000, 0x42);
    memory.write(0x1001, 0x55);
    memory.write(0x1002, 0xAA);

    // Read values from memory

    println!("Memory Test");

    println!(
        "0x1000 = {:02X}",
        memory.read(0x1000)
    );

    println!(
        "0x1001 = {:02X}",
        memory.read(0x1001)
    );

    println!(
        "0x1002 = {:02X}",
        memory.read(0x1002)
    );

    println!();

    cpu.dump();
}