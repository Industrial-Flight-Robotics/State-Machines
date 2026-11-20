pub const VRAM_SIZE: usize = 16 * 1024;

pub struct r9918 {
    vram: [u8; VRAM_SIZE],
    registers: [u8; 8],
    address: u16,
    status: u8,
}


impl r9918 {

    pub fn new() -> r9918 {

        r9918 {
            vram: [0; VRAM_SIZE],
            registers: [0; 8],
            address: 0,
            status: 0,
        }
    }

    pub fn write_vram(&mut self, address: u16, value: u8) {

        let address = (address & 0x3FFF) as usize;

        self.vram[address] = value;
    }


    pub fn read_vram(&self, address: u16) -> u8 {

        let index = (address & 0x3FFF) as usize;

        self.vram[index]
    }
}