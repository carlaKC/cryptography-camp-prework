#[derive(Debug)]
pub enum Error {
    NoPrimeModulo,
    ModuloDividesValue,
}

/// Uses Fermat's Little Theorem and fast exponentiation to calculate the modular inverse of a.
///
/// Errors if p is not prime, or p | a (which is a requirement for part two of FLT).
pub fn modular_inverse(a: u64, p: u64) -> Result<u64, Error> {
    // P must be prime for Fermat's Little Theorem to hold.
    if !primes::is_prime(p) {
        return Err(Error::NoPrimeModulo);
    }

    if a % p == 0 {
        return Err(Error::ModuloDividesValue);
    }

    // We know from FLT that if p | a then:
    // a^(p-1) ≡  1 (mod p)
    //
    // Rearranging this to a * b ≡ 1 (mod p), b will be the modular inverse of a:
    // a * a^(p-2) ≡  1 (mod p)
    //
    // Therefore we're aiming to calculate a^(p-2) to get the modular inverse, which we'll do with
    // fast exponentiation.
    Ok(exercise_1::fast_powering_algorithm(a, p - 2, p))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modular_inverse_non_prime_modulo() {
        assert!(matches!(modular_inverse(3, 4), Err(Error::NoPrimeModulo)));
        assert!(matches!(modular_inverse(2, 1), Err(Error::NoPrimeModulo)));
        assert!(matches!(modular_inverse(2, 0), Err(Error::NoPrimeModulo)));
    }

    #[test]
    fn test_modular_inverse_modulo_divides_value() {
        assert!(matches!(
            modular_inverse(10, 5),
            Err(Error::ModuloDividesValue)
        ));
        assert!(matches!(
            modular_inverse(0, 7),
            Err(Error::ModuloDividesValue)
        ));
        assert!(matches!(
            modular_inverse(21, 7),
            Err(Error::ModuloDividesValue)
        ));
    }

    #[test]
    fn test_modular_inverse_success() {
        // Test with small prime p = 7
        assert_eq!((2 * modular_inverse(2, 7).unwrap()) % 7, 1);
        assert_eq!((3 * modular_inverse(3, 7).unwrap()) % 7, 1);
        assert_eq!((5 * modular_inverse(5, 7).unwrap()) % 7, 1);

        // Test with larger prime p = 97
        assert_eq!((23 * modular_inverse(23, 97).unwrap()) % 97, 1);

        // Test edge case: inverse of 1 should be 1
        assert_eq!(modular_inverse(1, 7).unwrap(), 1);

        // Test edge case: inverse of p-1 should be p-1 (since (p-1)^2 ≡ 1 mod p)
        assert_eq!(modular_inverse(6, 7).unwrap(), 6);
    }
}
