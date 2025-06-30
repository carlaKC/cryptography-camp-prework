# Week 1 Notes

## Reading 1

- A set is a collection of objects
- Items in a set are called elements
- a ∈ S: a is an element in set S
- a ∉ S: a is not an element of set S
- S ⊆ T: S is a subset of T
  - if s ∈ S -> s ∈ T
- T ⊃ S: T contains S
  - This does not exclude the possibility that S = T
- To show that two sets are equal, prove:
 - S ⊆ T AND T ⊆ S
- ∅: Empty set symbol
  - ∅ ⊆ {any set}
- A ∪ B: union, elements in A, B or both
- A ∩ B: intersection, elements in A and B
- A - B: difference = { a ∈ A | a ∉ B }
- If A ⊆ S, S - A is the complement of A in S, written A'
- Universal intersections:
  - A ∩ A = A
  - A ∩ ∅ = ∅ 
  - If A ⊆ S, A ∩ S = A
- A = { s ∈ S | s satisfies P }
  - Describes a subset of S where all elements have a property P

Relation between operations:
A ∪ B = (A ∩ B) ∪ (A - B) ∪ (B - A)

To prove this, we'll demonstrate that each is a subset of the other.

We first show that:
(A ∩ B) ∪ (A - B) ∪ (B - A) ⊆ A ∪ B

By definition of each of these primatives:
(A ∩ B) ⊆ A
(A - B) ⊆ A
(B - A) ⊆ B

Therefore the combination of all three is ⊆ A ∪ B

Now we show that:
A ∪ B ⊆ (A ∩ B) ∪ (A - B) ∪ (B - A) 

Given u ∈ (A ∩ B), we enumerate all the possible placings of u:
if u ∈ A and u ∈ B -> u ∈ A ∩ B
if u ∈ A and u ∉ B -> u ∈ A - B
if u ∉ A and u ∈  B -> u B - A

These are all the possible values for u, it must be in A, B or both.

Therefore for every u ∈ (A ∩ B):
u ∈ (A ∩ B) ∪ (A - B) ∪ (B - A) 

We have proven subsets in both directions, so we have proven equality.

The Cartesian Product for two sets A and B:
A x B = { (a, b) | a ∈ A, b ∈  B}

Where we declare the ordered pair (a,b) to be equal to (a1, b1) 
iff a = a1 and b = b1.

This is the set of all possible unique pairs that can be made across
sets.

A function from S to T is a rule that assigns each element of S to
a unique element of T. To decide whether two mappings are equal for
S -> T we look at different classifications of functions f: S -> T:
- surjective:
  - every t ∈ T is the image under f of some s ∈ S
  - ie, for every t ∈ E, there exists s ∈ S where t = f(s)
- one-to-one:
  - if for s1 != s2 in S, f(s1) != f(s2)
  - f(s1) = f(s2) -> s1 = s2
  - This maps distinct objects to distinct images
- bijection: both one-to-one and surjective 

Note: Stopping on page 10, this is background and I haven't needed it
yet.

## Interlude

Public Key Function: G: Kpriv -> Kpub
Encryption Function: E: M x Kpub -> C
Decryption Function: D: C x Kpriv -> M

We will use p to denote a prime integer.
We're interested in a set of integers modulo a prive, ℤ/ℤp (or ℤp / Fp)

## Reading 2

Focus: an efficient algorithm for modular exponentiation.

We will be using the pubkey function:
g ∈ ℤp be a fixed non-zero number
G : ℤp -> ℤp is our pubkey function

G(x) = g^x mod p
     = g * g ... * g mod p (x times)

We define congruence for a, b, m ∈ ℤ:
a ≡ b (mod m) <-> m | a - b 

Read as: a is congruent to b mod m if m divides their difference.

When we have a zero congruence, we're expressing all of the numbers
that are multiples of m: a ≡ 0 (mod m), true for any a where m | a.

Proposition 1.13 (a): proven in remarkable sheet for the week.
- Addition and multiplication properties hold for congruences

Proposition 1.13 (b):

a * b ≡ 1 (mod m) for some interger b iff gcd(a,m) = 1

If b exists, it it is the multiplicative inverse of a modulo m.
There is only one inverse, as any two inverses are congruent mod m.

This is like the recipricol with fractions:
- The recipricol of 2/3 is 3/2, because it's the number you multiply
  it by to get 1.

In a * b ≡ 1 (mod m), b is the number that you multiply a by to make
it congruent modulo 1.

Proof: in remarkable sheet for the week.

We can use the euclidean algorithm to find the GCD of two numbers.
- It takes 2 log2(b) + 3 iterations to fund gcd(a,b)
- It takes a small multiple of log2(m) steps to find inverse

We express remainders as:
a = m * q + r, with 0 <= r < m

We therefore know that a ≡ r (mod m) because:
a - r = m * q
m | a - r
a ≡ r (mod m)

We express the ring of integers modulo m as:
ℤ/mℤ = {0, 1, 2, ... m -1}

Whenever we perform addition or multiplication in ℤm, we always
divide the result by m and take the remainder in order to obtain an
element in ℤm.

We know that a has an inverse modulo m iff (a, m) = 1. These values are
called units:

ℤ/mℤ* = { a ∈  ℤ/mℤ: gcd(a, m) = 1 }
ℤ/mℤ* = { a ∈  ℤ/mℤ: a has an inverse modulo m}

This is called the group of units modulo m.
- If a1 and a2 are units modulo m, a1 * a2 is a unit modulo m
- This is not the case for addition

Numbers that are coprime with m will be units, because their GCD is
one (ie, they have no common divisors).

We often want to know how many elements are in the set of units modulo
m. This is called Euler's phi function:

φ(n) = # (ℤ/mℤ*) = #{ 0 <= a <= m: gcd(a, m) = 1}
(# = count in set)

Fast multiplication:
- In cryptography applications, we often need to compute large powers
  of a number g modulo N, where N may be massive
- Naively, we can do this with repeated multiplications
  g1 ≡  g (mod N)
  g2 ≡  g * g1 (mod N)
  g3 ≡  g * g2 (mod N)
  ...
  ga ≡  g * g(a-1) (mod N)

Computing 2^1000 would take longer than the known age of the universe.

We can use binary expansion of the exponent A to simplify this to a
series of squares and multiplications eg: 3^218:
- Express 218 as a sum of powers of 2
- Express 3 to the power of those powers of 2
= 3 ^ 2 * 3 ^ (2^3) * 3 ^ (2^4) * 3 ^ (2^6) * 3 ^ (2^7)

Each exponent is the square of the previous one, so we can express
this quite simply.

When we compute the product, we can also take the modulo after each
multiplication so we never have to hold large values.

Steps in this algorithm:
1. Compute the binary expansion of A as:
   A = A0 + A1*2 + A2 * 2^2 ... + Ar * 2^r
2. Compute the powers of g^2^i (mod n) for 0 <= i < r
   Each term is the square of the previous one
3. Compute g^A using the formuala:
  g^A = g^( A0 + ... + Ar*2^r)
      = a0 * a1 ... ar^Ar (mod N)

Running time:
- It takes at most 2r multiplications modulo N to compute g^A. 
- Since A >= 2^r it takes at most 2log2(A) multiplications modulo N

It takes
## Reading 3a

- An integer is prime if p > 1 and the only positive integer divisors
  of p are 1 and p.
- Note that we could easily include 1 in the primes, it's our
  definition, but then we'd break things like prime exponentiation,
  so we choose to exclude it.

An element p of a ring is prime if:
- It is non-zero
- It has no multiplicative inverse
- If p | a * b -> p | a or p | b

The fundamental theorum of arithmetic:

Every positive integer can be expressed as a produce of primes, and
the factorization is unique.

Again, prime numbers technically only include one number in this
"product", be we choose to include these "degenerate" products to
suit our use of this theory in practice. Without allowing this
empty product, we'd also need a special carve out for 1, as it's
not a produce of primes, except for raising a prime to the power
of zero.

Proof: in remarkable sheet for the week

## Reading 3b

Division, Euclidean Algorithm and GCD: all covered in Furzey's course.

## Exercise 2

Find at least one proof of Fermat's little theorem online, and be
prepared to share your understanding.

## Fermat's Little Theorem

Let p be a positive prime.
For ∀ a ∈ Z:

1) a^p ≡ a (mod p)
2) If p does not divide a -> a^(p-1) ≡ 1 (mod p)

## Reading 4

Symmetric Ciphers:
- Shared secret key k
- Used for both encryption and decryption:
  - e: K x M -> C
  - d: K x C -> M
  - Obviously: d(k, e(k, m)) = m (decryption of encryption = message)
    Sometimes shortened to: dk(ek(m)) = m for all m ∈ M
- dk is the inverse function of ek
  -> This means that ek must be one-to-one

Kerckhoff's Principle: the security of a crypto system should depend
only on the secrecy of the key, and not secrecy of the algorithm itself.

To be a successful ciper (K, M, C, e, d) must:
- For k ∈ K, m ∈ M, it must be easy to compute ciphertext ek(m)
- For k ∈ K, c ∈ C, it must be easy to compute the plaintest dk(c)
- Given c1, c2 .. ∈ C encrypted using k ∈ K, it must be very difficult
  to compute any of the corresponding plaintexts dk(c1), dk(c2).. 
  without knowledge of K

Another property that is desirable but more difficult to achieve is
security against a chosen plaintext attack:
- Given pairs of plaintexts and ciphertexts (m1, c1), (m2, c2)...
  it must be difficult to decrypt any ciphertext c that is not in the
  given list without knowing k.

The encoding scheme is a method of converting one sort of data into
another, eg characters to bytes (like ASCII). This scheme is known by
everyone and used in the same way by everyone. It is convenient to view
the plaintext space M as consisting of bit strings of a fixed length B,
called the blocksize of the cipher. A plaintext message then consists of
a list of message blocks chosen from M.

- Bk = blocksize for keys
  K = { k ∈ ℤ : 0 <= k < 2^Bk }
- Bm = blocksize for messages
  M = { m ∈ ℤ : 0 <= m < 2^Bm }
- Bc = blocksize for ciphertext
  C = { c ∈ ℤ : 0 <= m < 2^Bc }

You need to choose Bk so that the possible values for K aren't trivially
enumerable, otherwise an attacker can easily just check every possible
key (brute force / exhaustive search).

Resume on section 1.7.4
