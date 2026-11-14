pub struct R80{
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub pc: u16,
    pub sp: u16,
}

impl R80 {

    pub fn new() -> R80 {

        R80 {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            h: 0,
            l: 0,

            pc: 0,
            sp: 0,
        }
    }

    pub fn dump(&self) {

            println!("Intel 8080 CPU Registers");

            println!("A  = {:02X}", self.a);
            println!("B  = {:02X}", self.b);
            println!("C  = {:02X}", self.c);
            println!("D  = {:02X}", self.d);
            println!("E  = {:02X}", self.e);
            println!("H  = {:02X}", self.h);
            println!("L  = {:02X}", self.l);

            println!("PC = {:04X}", self.pc);
            println!("SP = {:04X}", self.sp);
    }
}