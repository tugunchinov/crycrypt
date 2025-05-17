use crate::utils::safe_power_modulo;
use std::num::NonZeroU32;

const P: u32 = 58217;
const Q: u32 = 31957;

const N: NonZeroU32 = NonZeroU32::new(P * Q).unwrap();
const PHI: u32 = (P - 1) * (Q - 1);

const fn compute_e() -> u32 {
    let mut i = 3;
    loop {
        if i % PHI != 0 {
            return i;
        }
        i += 2;
    }
}

const E: u32 = compute_e();

pub struct RsaPrg {
    state: u32,
}

impl RsaPrg {
    pub fn new(seed: u32) -> Self {
        let seed = seed % N;

        Self { state: seed }
    }

    pub fn gen_u8(&mut self) -> u8 {
        let mut result = 0;

        for i in 0..u8::BITS {
            self.state = safe_power_modulo(self.state, E, N);
            result |= (self.state & 1) << i;
        }

        result as u8
    }
}

#[cfg(test)]
mod test {
    use crate::prg::rsa::RsaPrg;

    #[test]
    fn playground() {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u32;

        let mut prg = RsaPrg::new(seed);

        println!("Pseudorandom sequence:");
        for _ in 0..100 {
            println!("{}", prg.gen_u8());
        }
    }

    #[test]
    fn statistics() {
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u32;

        let mut prg = RsaPrg::new(seed);

        let mut cnts = std::collections::BTreeMap::new();

        for _ in 0..1_000_000 {
            let val = prg.gen_u8();
            *cnts.entry(val).or_insert(0) += 1;
        }

        println!("Statistics:");

        let min = cnts.values().min().unwrap();
        let max = cnts.values().max().unwrap();

        println!("Min: {}\nMax: {}", min, max);

        for (val, cnt) in cnts {
            println!("{}: {}", val, cnt);
        }
    }
}
