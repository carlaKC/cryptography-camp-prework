use rand::Rng;

#[derive(Debug)]
pub enum Error {
    /// Randomly generated key does not fall within requested bounds.
    KeyOutOfBounds,
    /// Message is too large for chosen prime.
    MessageOutOfBounds,
    /// Could not calculate the modular inverse of a value.
    ModularInverseFailure,
}

/// The publicly defined parameters for an ElGamalCipher, used in conjunction with a provided
/// pubkey to encrypt messages.
#[derive(Copy, Clone)]
pub struct ElGamalParams {
    generator: u64,
    prime: u64,
}

impl ElGamalParams {
    pub fn el_gamal_encrypt(
        &self,
        eph_key: u64,
        pubkey: u64,
        message: u64,
    ) -> Result<(u64, u64), Error> {
        if !self.validate_range(message) {
            return Err(Error::MessageOutOfBounds);
        }

        if !self.validate_range(eph_key) {
            return Err(Error::KeyOutOfBounds);
        }

        let c1 = exercise_1::fast_powering_algorithm(self.generator, eph_key, self.prime);
        let c2 =
            message * exercise_1::fast_powering_algorithm(pubkey, eph_key, self.prime) % self.prime;
        Ok((c1, c2))
    }

    /// Generates a random key that is within 1 <= key <= prime -2.
    fn random_key(&self) -> Result<u64, Error> {
        let key = rand::thread_rng().gen_range(1..=(self.prime - 2));
        if !self.validate_range(key) {
            return Err(Error::KeyOutOfBounds);
        }
        Ok(key)
    }

    /// Validates that a value remains within our field size, returning a boolean if the value is
    /// valid.
    fn validate_range(&self, val: u64) -> bool {
        val >= 1 && val <= self.prime - 2
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
    pub fn new(params: ElGamalParams, privkey: u64) -> Result<Self, Error> {
        if !params.validate_range(privkey) {
            return Err(Error::KeyOutOfBounds);
        }

        Ok(ElGamalCipher {
            privkey,
            pubkey: exercise_1::fast_powering_algorithm(params.generator, privkey, params.prime),
            params,
        })
    }

    pub fn decrypt(&self, ciphertext: (u64, u64)) -> Result<u64, Error> {
        let x = exercise_1::fast_powering_algorithm(ciphertext.0, self.privkey, self.params.prime);
        let x_1 = exercise_3::modular_inverse(x, self.params.prime)
            .map_err(|_| Error::ModularInverseFailure)?;

        Ok(ciphertext.1 * x_1 % self.params.prime)
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
    fn test_default_params() {
        let params = ElGamalParams::default();
        let privkey = params.random_key().unwrap();
        let cipher = ElGamalCipher::new(params, privkey).unwrap();
        let eph_key = params.random_key().unwrap();

        let ciphertext = params
            .el_gamal_encrypt(eph_key, cipher.pubkey(), 3)
            .unwrap();
        assert_eq!(cipher.decrypt(ciphertext).unwrap(), 3);
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let test_params = [
            ElGamalParams {
                generator: 33,
                prime: 71,
            },
            ElGamalParams {
                generator: 11,
                prime: 23,
            },
            ElGamalParams {
                generator: 3,
                prime: 809,
            },
            ElGamalParams {
                generator: 6,
                prime: 17,
            },
        ];

        for params in test_params {
            let privkey = params.random_key().unwrap();
            let cipher = ElGamalCipher::new(params, privkey).unwrap();
            let eph_key = params.random_key().unwrap();

            let test_messages = [1, 5, 10, params.prime - 2];

            for &message in &test_messages {
                let ciphertext = params
                    .el_gamal_encrypt(eph_key, cipher.pubkey(), message)
                    .unwrap();
                let decrypted = cipher.decrypt(ciphertext).unwrap();
                assert_eq!(
                    decrypted, message,
                    "Roundtrip failed for message {} with params (g={}, p={})",
                    message, params.generator, params.prime
                );
            }
        }
    }

    #[test]
    fn test_message_out_of_bounds() {
        let params = ElGamalParams::default();
        let privkey = params.random_key().unwrap();
        let cipher = ElGamalCipher::new(params, privkey).unwrap();
        let eph_key = params.random_key().unwrap();

        let result = params.el_gamal_encrypt(eph_key, cipher.pubkey(), 0);
        assert!(matches!(result, Err(crate::Error::MessageOutOfBounds)));

        let result = params.el_gamal_encrypt(eph_key, cipher.pubkey(), params.prime - 1);
        assert!(matches!(result, Err(crate::Error::MessageOutOfBounds)));

        let result = params.el_gamal_encrypt(eph_key, cipher.pubkey(), params.prime);
        assert!(matches!(result, Err(crate::Error::MessageOutOfBounds)));

        let result = params.el_gamal_encrypt(eph_key, cipher.pubkey(), params.prime + 100);
        assert!(matches!(result, Err(crate::Error::MessageOutOfBounds)));
    }
}
