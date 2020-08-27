mod bitconverter;
mod pk8;
mod seedsearchutil;
mod xoroshiro128plus;
use boolector::option::{BtorOption, ModelGen};
use boolector::{Btor, SolverResult, BV};
use pk8::PK8;
use std::{env, rc::Rc};
use xoroshiro128plus::{XoroShiro, XOROSHIRO_CONST};

fn find_potential_seeds(ec: u32, pid: u32, tid: u32, sid: u32, shiny: u32) -> Vec<u64> {
    let btor = Rc::new(Btor::new());
    btor.set_opt(BtorOption::ModelGen(ModelGen::Asserted));
    btor.set_opt(BtorOption::Incremental(true));

    let mut sym_s0 = BV::new(btor.clone(), 64, Some("start_s0"));
    let mut sym_s1 = BV::from_u64(btor.clone(), XOROSHIRO_CONST, 64);

    let zero = BV::from_u32(btor.clone(), 0, 64);
    let sixteen = BV::from_u32(btor.clone(), 16, 64);
    let and_val16 = BV::from_u32(btor.clone(), 0xFFFF, 64);

    let real_ec = BV::from_u32(btor.clone(), ec, 64);
    let real_pid = BV::from_u32(btor.clone(), pid, 64);
    let real_tid = BV::from_u32(btor.clone(), tid, 64);
    let real_sid = BV::from_u32(btor.clone(), sid, 64);

    let sym_ec = advance_symbolic(&btor, &mut sym_s0, &mut sym_s1);
    let sym_sidtid = advance_symbolic(&btor, &mut sym_s0, &mut sym_s1);
    let mut sym_pid = advance_symbolic(&btor, &mut sym_s0, &mut sym_s1);
    sym_ec._eq(&real_ec).assert();

    let sym_sidtid_shr16 = BV::srl(&sym_sidtid, &sixteen);
    let sym_sidtid_andval16 = BV::and(&sym_sidtid, &and_val16);
    let tmp_sidtid = BV::xor(&sym_sidtid_shr16, &sym_sidtid_andval16);

    let sym_pid_shr16 = BV::srl(&sym_pid, &sixteen);
    let sym_pid_andval16 = BV::and(&sym_pid, &and_val16);
    let tmp_pid = BV::xor(&sym_pid_shr16, &sym_pid_andval16);

    let sym_shiny = BV::xor(&tmp_sidtid, &tmp_pid);
    match shiny {
        0 => sym_shiny.ugte(&sixteen).assert(),
        1 => sym_shiny.ult(&sixteen).assert(),
        _ => sym_shiny._eq(&zero).assert(),
    }

    if shiny != 0 {
        let tmp1 = BV::xor(&sym_pid_andval16, &real_tid);
        let tmp2 = BV::xor(&real_sid, &BV::from_u32(btor.clone(), 2 - shiny, 64));
        let high = BV::xor(&tmp1, &tmp2);
        let high_shl16 = BV::sll(&high, &sixteen);
        sym_pid = BV::or(&high_shl16, &sym_pid_andval16);
    }
    sym_pid._eq(&real_pid).assert();

    get_results(&btor)
}

fn get_results(ctx: &Rc<Btor>) -> Vec<u64> {
    let mut results: Vec<u64> = Vec::new();
    let s0 = Btor::get_bv_by_symbol(ctx.clone(), "start_s0").unwrap();
    while ctx.sat() == SolverResult::Sat {
        let val = s0.get_a_solution();
        results.push(val.clone().as_u64().unwrap());
        s0._ne(&BV::from_binary_str(ctx.clone(), val.as_01x_str()))
            .assert();
    }
    results
}

fn advance_symbolic(
    ctx: &Rc<Btor>,
    sym_s0: &mut BV<Rc<Btor>>,
    sym_s1: &mut BV<Rc<Btor>>,
) -> BV<Rc<Btor>> {
    let and_val = BV::from_u64(ctx.clone(), 0xFFFFFFFF, 64);
    let sym_r = BV::and(&BV::add(sym_s0, sym_s1), &and_val);
    *sym_s1 = BV::xor(sym_s0, sym_s1);
    let twenty_four = BV::from_u64(ctx.clone(), 24, 64);
    let tmp = BV::rol(sym_s0, &twenty_four);
    let tmp2 = BV::from_u64(ctx.clone(), 1 << 16, 64);
    *sym_s0 = BV::xor(&tmp, &BV::xor(sym_s1, &BV::mul(sym_s1, &tmp2)));
    let thirty_seven = BV::from_u64(ctx.clone(), 37, 64);
    *sym_s1 = BV::rol(sym_s1, &thirty_seven);
    sym_r
}

fn find_valid_seeds(seeds: Vec<u64>, ivs: [u8; 6]) -> Vec<u64> {
    let mut results = Vec::<u64>::new();
    for seed in seeds {
        for iv_count in 1..6 {
            let mut rng = XoroShiro::new(seed);

            rng.next_int(0xFFFFFFFF);
            rng.next_int(0xFFFFFFFF);
            rng.next_int(0xFFFFFFFF);
            let mut check_ivs: [u8; 6] = [32; 6];
            let mut count = 0;
            while count < iv_count {
                let stat = rng.next_int(6) as usize;
                if check_ivs[stat] == 32 {
                    check_ivs[stat] = 31;
                    count += 1;
                }
            }
            for iv in &mut check_ivs {
                if *iv == 32 {
                    *iv = rng.next_int(32) as u8;
                }
            }
            if ivs == check_ivs {
                results.push(seed);
            }
        }
    }
    results
}

fn search(pk8: &PK8) -> Option<Vec<u64>> {
    let ec = pk8.get_ec();
    let mut pid = pk8.get_pid();
    let tid = pk8.get_tid();
    let sid = pk8.get_sid();
    let ivs = pk8.get_ivs();
    for flag in [false, true].iter() {
        if *flag {
            pid ^= 0x10000000;
        }

        let shiny = seedsearchutil::get_shiny_type(tid, sid, pid);
        let seeds = find_potential_seeds(ec, pid, tid, sid, shiny);
        if !seeds.is_empty() {
            let valid_seeds = find_valid_seeds(seeds, ivs);
            if !valid_seeds.is_empty() {
                return Some(valid_seeds);
            }
        }
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    for arg in args {
        let bytes: Vec<u8> = std::fs::read(&arg).expect("File not found.");
        let pk8 = PK8::new(bytes);
        let seeds = search(&pk8).unwrap_or_default();
        println!("{}:", arg);
        if !seeds.is_empty() {
            for seed in seeds {
                println!("{:X}", seed);
            }
            println!();
        } else {
            println!("No raid seed.");
        }
    }
}
