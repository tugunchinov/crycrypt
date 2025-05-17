use crate::utils::safe_power_modulo;
use std::num::NonZeroU32;

const P: u32 = 1733;
const Q: u32 = 319919;

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
        for _ in 0..10 {
            println!("{}", prg.gen_u8());
        }
    }
}
