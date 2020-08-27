use crate::bitconverter;

#[allow(non_snake_case)]
pub struct PK8 {
    data: Vec<u8>,
}

impl PK8 {
    pub fn new(data: Vec<u8>) -> PK8 {
        PK8 { data }
    }

    pub fn get_ec(&self) -> u32 {
        bitconverter::to_uint32(&self.data, 0x0)
    }

    pub fn get_tid(&self) -> u32 {
        bitconverter::to_uint16(&self.data, 0xC) as u32
    }

    pub fn get_sid(&self) -> u32 {
        bitconverter::to_uint16(&self.data, 0xE) as u32
    }

    pub fn get_pid(&self) -> u32 {
        bitconverter::to_uint32(&self.data, 0x1C)
    }

    fn get_iv(&self) -> u32 {
        bitconverter::to_uint32(&self.data, 0x8C)
    }

    fn get_hp(&self) -> u8 {
        (self.get_iv() & 0x1F) as u8
    }

    fn get_atk(&self) -> u8 {
        ((self.get_iv() >> 5) & 0x1F) as u8
    }

    fn get_def(&self) -> u8 {
        ((self.get_iv() >> 10) & 0x1F) as u8
    }

    fn get_speed(&self) -> u8 {
        ((self.get_iv() >> 15) & 0x1F) as u8
    }

    fn get_sp_a(&self) -> u8 {
        ((self.get_iv() >> 20) & 0x1F) as u8
    }

    fn get_sp_d(&self) -> u8 {
        ((self.get_iv() >> 25) & 0x1F) as u8
    }

    pub fn get_ivs(&self) -> [u8; 6] {
        [
            self.get_hp(),
            self.get_atk(),
            self.get_def(),
            self.get_sp_a(),
            self.get_sp_d(),
            self.get_speed(),
        ]
    }
}
