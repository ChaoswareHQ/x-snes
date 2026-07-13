pub const FLAG_CARRY: u8 = 0b0000_0001; // C
pub const FLAG_ZERO: u8 = 0b0000_0010; // Z
pub const FLAG_INTERRUPT: u8 = 0b0000_0100; // I
pub const FLAG_DECIMAL: u8 = 0b0000_1000; // D
pub const FLAG_INDEX_8: u8 = 0b0001_0000; // X (0 = 16-bit, 1 = 8-bit) / Break in Emulation
pub const FLAG_MEM_8: u8 = 0b0010_0000; // M (0 = 16-bit, 1 = 8-bit)
pub const FLAG_OVERFLOW: u8 = 0b0100_0000; // V
pub const FLAG_NEGATIVE: u8 = 0b1000_0000; // N

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Cpu5A22 {
    // Layout:
    // 0..2   => PC (u16)
    // 2..4   => A  (u16)
    // 4..6   => X  (u16)
    // 6..8   => Y  (u16)
    // 8..10  => S  (u16 - Stack Pointer)
    // 10..12 => D  (u16 - Direct Page)
    // 12     => DBR (u8  - Data Bank Register)
    // 13     => PBR (u8  - Program Bank Register)
    // 14     => SR  (u8  - Status Register 'P')
    // 15     => E   (u8  - Emulation mode flag: 1 = Emulation, 0 = Native)
    bytes: [u8; 16],
}
