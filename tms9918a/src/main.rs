mod r9918;

use r9918::R9918;


fn main() {

    println!("TMS9918 Emulator Starting...");

    let mut vdp = R9918::new();

    // Write data into VRAM
    vdp.write_vram(0x1000, 0xAA);

    // Read the data back
    let value = vdp.read_vram(0x1000);

    println!("VRAM[0x1000] = 0x{:02X}", value);
}