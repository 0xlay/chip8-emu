pub struct Registers {
    pub PC: u16, // current instruction in memory
    pub I: u16,  // index register
    pub ST: u8,  // sound timer
    pub DT: u8,  // delay timer
    pub V0: u8,
    pub V1: u8,
    pub V2: u8,
    pub V3: u8,
    pub V4: u8,
    pub V5: u8,
    pub V6: u8,
    pub V7: u8,
    pub V8: u8,
    pub V9: u8,
    pub VA: u8,
    pub VB: u8,
    pub VC: u8,
    pub VD: u8,
    pub VE: u8,
    pub VF: u8,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            PC: 0x200,
            I: 0,
            ST: 60,
            DT: 50,
            V0: 0,
            V1: 0,
            V2: 0,
            V3: 0,
            V4: 0,
            V5: 0,
            V6: 0,
            V7: 0,
            V8: 0,
            V9: 0,
            VA: 0,
            VB: 0,
            VC: 0,
            VD: 0,
            VE: 0,
            VF: 0,
        }
    }
}
