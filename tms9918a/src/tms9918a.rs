pub const VRAM_SIZE: usize = 16 * 1024;

pub struct tms9918a {
    vram: [u8; VRAM_SIZE],

    registers: [u8; 8],

    address: u16,

    status: u8,
}


impl tms9918a {

    pub fn new() -> tms9918a {

        tms9918a {
            vram: [0; VRAM_SIZE],

            registers: [0; 8],

            address: 0,

            status: 0,
        }
    }

}