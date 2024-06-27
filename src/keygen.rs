use num_primes::{BigUint, Generator};
use crate::core::Paillier;

#[derive(Debug)]
pub(crate) struct PublicKey {
    pub(crate) n: BigUint,
    pub(crate) g: BigUint
}

#[derive(Debug)]
pub(crate) struct PrivateKey {
    pub(crate) lambda: BigUint,
    pub(crate) mu: BigUint
}

impl Paillier {

    pub(crate) fn gen_key(n_length: usize) -> (PublicKey, PrivateKey) {

        // 生成p q大素数
        let mut p: BigUint;
        let mut q: BigUint;
        loop {
            let a = Generator::new_prime(n_length / 2);
            let b = Generator::new_prime(n_length / 2);
            // 要求 p != q && q < p
            if !a.eq(&b) {
                if a.lt(&b) {
                    p = a.clone();
                    q = b.clone();
                } else {
                    p = b.clone();
                    q = a.clone();
                }
                break;
            }
        }

        assert_eq!(p.bits(), q.bits(), "p.bits != q.bits");
        let n = &p * &q;
        let g = &n + (1 as usize);

        let public_key = PublicKey { n, g };
    }
}

