use rand::RngCore;

/// The publicly defined parameters for an ElGamalCipher, used in conjunction with a provided
/// pubkey to encrypt messages.
#[derive(Copy, Clone)]
pub struct ElGamalParams {
    generator: u64,
    prime: u64,
}

impl ElGamalParams {
    pub fn el_gamal_encrypt(&self, pubkey: u64, message: u64) -> (u64, u64) {
        let nonce = rand::thread_rng().next_u64() % self.prime;
        let c1 = exercise_1::fast_powering_algorithm(self.generator, nonce, self.prime);
        let c2 =
            message * exercise_1::fast_powering_algorithm(pubkey, nonce, self.prime) % self.prime;
        (c1, c2)
    }
}

impl Default for ElGamalParams {
    /// Sane defaults for a test/toy cipher.
    fn default() -> Self {
        ElGamalParams {
            generator: 2,
            prime: 467,
        }
    }
}

/// The private cipher that is used to decrypt messages.
pub struct ElGamalCipher {
    privkey: u64,
    pubkey: u64,
    params: ElGamalParams,
}

impl ElGamalCipher {
    /// Randomly generates a private key and creates a cipher with the parameters provided.
    pub fn new(params: ElGamalParams) -> Self {
        let privkey = rand::thread_rng().next_u64() % params.prime; // TODO: < p-2
        let pubkey = exercise_1::fast_powering_algorithm(params.generator, privkey, params.prime);

        ElGamalCipher {
            privkey,
            pubkey,
            params,
        }
    }

    pub fn decrypt(&self, ciphertext: (u64, u64)) -> u64 {
        let x = exercise_1::fast_powering_algorithm(ciphertext.0, self.privkey, self.params.prime);
        let x_1 = exercise_3::modular_inverse(x, self.params.prime).unwrap();

        ciphertext.1 * x_1 % self.params.prime
    }

    /// Returns the pubkey to be used for message encryption.
    pub fn pubkey(&self) -> u64 {
        self.pubkey
    }
}

#[cfg(test)]
mod tests {
    use crate::{ElGamalCipher, ElGamalParams};

    #[test]
    fn test_basic() {
        let params = ElGamalParams::default();
        let cipher = ElGamalCipher::new(params);

        let ciphertext = params.el_gamal_encrypt(cipher.pubkey(), 3);
        assert_eq!(cipher.decrypt(ciphertext), 3);
    }
}
