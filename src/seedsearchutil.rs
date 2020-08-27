#![allow(dead_code)]
use crate::xoroshiro128plus::XoroShiro;

fn get_shiny_xor(val: u64) -> u64 {
    (val >> 16) ^ (val & 0xFFFF)
}

pub fn get_shiny_type(tid: u32, sid: u32, pid: u32) -> u32 {
    let tsv = tid ^ sid;
    let psv = (pid >> 16) ^ (pid & 0xffff);

    if tsv == psv {
        2 // Square
    } else if (tsv ^ psv) < 16 {
        1 // Star
    } else {
        0 // Not shiny
    }
}

pub fn get_shiny_value(val: u64) -> u64 {
    get_shiny_xor(val) >> 4
}

pub fn get_next_shiny_frame(seed: u64, shiny_type: u32) -> u64 {
    let mut rng = XoroShiro::new(seed);
    let mut count = 0;
    let mut seed = seed;
    loop {
        let _ = rng.next_int(0xFFFFFFFF);
        let sidtid = rng.next_int(0xFFFFFFFF) as u32;
        let pid = rng.next_int(0xFFFFFFFF) as u32;
        let type_shiny = get_shiny_type(sidtid, sidtid, pid);
        if type_shiny == shiny_type {
            return count;
        }
        rng = XoroShiro::new(seed);
        seed = rng.next();
        rng = XoroShiro::new(seed);
        count += 1;
    }
}
