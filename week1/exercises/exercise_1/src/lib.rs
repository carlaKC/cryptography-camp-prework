/// Performs binary expansion of n, expressing the number as the sum of powers of two.
/// The vector returned is minimal size for the n provided, where the boolean at each index i
/// indicates whether 2^i should be included in the expansion.
///
/// ```
/// # use fast_powering_algorithm::binary_expansion;
/// let result = binary_expansion(13);
/// assert_eq!(result, vec![true, false, true, true]); // 2^0 + 2^2 + 2^3 = 1 + 4 + 8 = 13
/// ```
pub fn binary_expansion(n: u64) -> Vec<bool> {
    if n == 0 {
        return vec![false];
    }

    // We can trim off any leading zeros, these will all be expressed as zero powers anyway.
    let n_len = 64 - n.leading_zeros() as usize;
    let mut exponents = Vec::with_capacity(n_len);

    let mut num = n;
    while num > 0 {
        exponents.push((num & 1) == 1);
        num = num >> 1;
    }

    // The value should always equal 2 raised to the power of the true exponents.
    debug_assert_eq!(
        n,
        exponents
            .iter()
            .enumerate()
            .filter(|bit| *bit.1)
            .map(|i| 2_u64.pow(i.0 as u32))
            .sum()
    );
    exponents
}

/// Takes a base number and the binary expansion of the exponent that it is being raised to and
/// returns a vector of the values n raised to the power of 2^ binary expansion.

/// Computes the successive powers of two for the number provided, given the binary expansion of
/// its exponent.
///
/// ```
/// # use fast_powering_algorithm::compute_successive_squares;
/// let result = compute_successive_squares(3, vec![true, false, true]);
/// let expected = vec![3_u64.pow(1), 3_u64.pow(4)];
/// assert_eq!(result, expected);
/// ```
pub fn compute_successive_squares(n: u64, exponent_bin_expansion: Vec<bool>) -> Vec<u64> {
    assert!(
        exponent_bin_expansion.len() <= 64,
        "only considering u64 integers"
    );

    let mut val = n;
    let mut multiplications = 0;
    let res = exponent_bin_expansion
        .iter()
        .filter_map(|include_product| {
            let res = if *include_product { Some(val) } else { None };

            val = val.pow(2);
            multiplications += 1;
            res
        })
        .collect();

    // We should never perform more multiplication operations than the length of our exponent list.
    assert!(multiplications <= exponent_bin_expansion.len());

    res
}

/// Performs the fast powering algorithm to calculate: base ^ exponent (mod modulus).
#[allow(dead_code)]
fn fast_powering_algorithm(base: u64, exponent: u64, modulo: u64) -> u64{
    let exponent_powers = binary_expansion(exponent);
    let successive_squares = compute_successive_squares(base, exponent_powers);

	successive_squares.iter().fold(1u64, |acc, &x| (acc * x) % modulo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_expansion_zero() {
        assert_eq!(binary_expansion(0), vec![false]);
    }

    #[test]
    fn test_binary_expansion_one() {
        assert_eq!(binary_expansion(1), vec![true]);
    }

    #[test]
    fn test_binary_expansion_powers_of_two() {
        // 2 = 2^1
        assert_eq!(binary_expansion(2), vec![false, true]);
        // 4 = 2^2
        assert_eq!(binary_expansion(4), vec![false, false, true]);
        // 8 = 2^3
        assert_eq!(binary_expansion(8), vec![false, false, false, true]);
    }

    #[test]
    fn test_binary_expansion_mixed() {
        // 5 = 2^0 + 2^2 = 1 + 4
        assert_eq!(binary_expansion(5), vec![true, false, true]);
        // 10 = 2^1 + 2^3 = 2 + 8
        assert_eq!(binary_expansion(10), vec![false, true, false, true]);
        // 13 = 2^0 + 2^2 + 2^3 = 1 + 4 + 8
        assert_eq!(binary_expansion(13), vec![true, false, true, true]);
    }

    #[test]
    fn test_binary_expansion_max_u64() {
        // u64::MAX should set all bits
        let result = binary_expansion(u64::MAX);
        assert_eq!(result.len(), 64);
        assert!(result.iter().all(|&bit| bit));
    }

    #[test]
    fn test_binary_expansion_highest_bit() {
        // 2^63 is the highest bit in u64
        let highest_bit = 1u64 << 63;
        let result = binary_expansion(highest_bit);
        assert_eq!(result.len(), 64);
        // Only the 64th bit (index 63) should be true
        assert!(result[63]);
        assert!(result[0..63].iter().all(|&bit| !bit));
    }

    #[test]
    fn test_fast_powering_algorithm_basic() {
        // 2^3 mod 5 = 8 mod 5 = 3
        assert_eq!(fast_powering_algorithm(2, 3, 5), 3);
        // 3^4 mod 7 = 81 mod 7 = 4
        assert_eq!(fast_powering_algorithm(3, 4, 7), 4);
        // 5^0 mod 11 = 1
        assert_eq!(fast_powering_algorithm(5, 0, 11), 1);
        // 7^1 mod 13 = 7
        assert_eq!(fast_powering_algorithm(7, 1, 13), 7);
    }

    #[test]
    fn test_fast_powering_algorithm_large() {
        // 2^10 mod 1000 = 1024 mod 1000 = 24
        assert_eq!(fast_powering_algorithm(2, 10, 1000), 24);
        // 3^20 mod 1000000 = 3486784401 mod 1000000 = 784401
        assert_eq!(fast_powering_algorithm(3, 20, 1000000), 784401);
    }
}
