pub struct XoroShiro {
    s0: u64,
    s1: u64,
}

pub const XOROSHIRO_CONST: u64 = 0x82A2B175229D6A5B;

impl XoroShiro {
    pub fn new(seed: u64) -> XoroShiro {
        XoroShiro {
            s0: seed,
            s1: XOROSHIRO_CONST,
        }
    }

    fn rotl(self: &Self, x: u64, k: u32) -> u64 {
        x.rotate_left(k)
    }

    /// Gets the next random `u64`
    pub fn next(&mut self) -> u64 {
        let result = u64::wrapping_add(self.s0, self.s1);
        self.s1 ^= self.s0;
        self.s0 = self.rotl(self.s0, 24) ^ self.s1 ^ (self.s1 << 16);
        self.s1 = self.rotl(self.s1, 37);
        result
    }

    /// Gets a random value that is less than `mod`
    #[allow(non_snake_case)]
    pub fn next_int(&mut self, MOD: u64) -> u64 {
        let mask = MOD.next_power_of_two() - 1;
        let mut result;
        while {
            result = self.next() & mask;
            result >= MOD
        } {}
        result
    }
}
