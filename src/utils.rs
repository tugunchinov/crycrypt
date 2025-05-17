use std::num::NonZeroU32;

pub fn safe_power_modulo(x: u32, mut exp: u32, modulus: NonZeroU32) -> u32 {
    if x == 0 {
        return 1;
    };

    let mut x = x as u64;
    let mut y = 1u64;
    let modulus = modulus.get() as u64;
    while exp > 1 {
        if exp & 1 == 1 {
            y = (y * x) % modulus;
            exp -= 1;
        }

        x = (x * x) % modulus;
        exp >>= 1;
    }

    ((x * y) % modulus) as u32
}

#[cfg(test)]
mod tests {
    use crate::utils::safe_power_modulo;
    use std::num::NonZeroU32;

    #[test]
    fn it_works() {
        assert_eq!(
            safe_power_modulo(2, 10, NonZeroU32::new(1025).unwrap()),
            1024
        );
        assert_eq!(safe_power_modulo(2, 10, NonZeroU32::new(7).unwrap()), 2);
    }
}
