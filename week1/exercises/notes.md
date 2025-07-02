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

Let p be a large prime, 2^159 < p < 2^160.
Let K = M = C = { 1, 2, ... p -1 } = 𝔽p (in the finite field).
- Randomly select key: k ∈ K, and calculate its inverse k'
- Encryption function: ek(m) ≡ k * m (mod p)
- Decryption function: dk(c) ≡ k' * c (mod p)

The encryption function is surjective for any choice of key k:
- For every c ∈ C and k ∈ K, there exists m ∈  M such that ek(m) = c
- Given any ciphertext c ∈ C and any plaintext m ∈ M, there exists a
  key k such that ek(m) = c
- This is true for the key: k ≡ m^-1 * c (mod p)

This does not have security against the chosen plaintext attack, because
with a single plaintext/ciphertext pair (m, c) the key k can be found.

Affine Cipher:
- ek(m) = k1 * m + k2 (mod p)
- dk(m) = k1' * (c - k2) (mod p)

Asymmetric Encryption:
- k = (kpriv, kpub)
- ekpub: M -> C
- dkpriv: C -> M
  - It must be difficult to compute this even if you know kpub
- kpriv(ekpub(m)) = M for all m ∈ M

## Abstract Algebra Overview

- Groups are "interfaces" consisting of:
  - A set of elements, X
  - A group operation, which takes two elements X and produces another X
- They must meet the following criteria:
  - Closed: the group operation always maps to a member of X
  - Identity: there must be a identity element:
    - Additive: a + i = a
    - Multiplicative: a * i = a
  - Associative:
    - Additive: a + (b + c) = (a + b) + c
    - Multiplicative: a * (b * c) = (a * b) * c
  - Inverse: for every a in X, there exists x in X such that:
    - Additive (-a): a + x = 0
    - Multiplicative (a^-1): a * x = 1

We'll primarily be concerned with groups expressed:
- Additively:
  - Operation: +
  - Identity: 0
  - Inverse: -a
- Multiplicativly:
  - Operation: X
  - Identity: 1
  - Inverse: a^-1

The number of elements in a group is called its order:
- May be infinite
- Must be > 0 (as we need 1 element for identity at least)

Abelian groups:
- Also have the property of commutativity:
  - Additive: a + b = b + a
  - Multiplicative: a * b = b * a

A group isomorphism is a bijective function that preserves the group
structure:
- Maps the identity element in one group to another
- Preserves the group operation

If an isomorphism exists between two groups, they are isomorphic.
They essentially "behave" the same, but just have different values
under the hood.

A group is a subgroup if for a subset of elements the group operation
still holds.
- Lagrange's theorem states that the order of a subgroup always
  divides the order of the group it originated from.
- Given a group G and an element g in the subgroup generated by g
  ⟨g⟩ consists of:
  - Additively: { 0, g, 2g, 3g ... }
  - Multiplicativly: {1, g, g^2, g^3 ... }
- If a group has a single element that generates the entire group, ie:
  - ⟨g⟩ = G
  - The group is cyclic
  - G is the generator

We denote repeated application of the group operation n times as:
- Additive: n * x
- Multiplicative: x^n

Note that here n is a non-negative interger, not a member of the group.

We define the discrete logarithm logb(a) or DLb(a):
- For element a and b in the group
- The smallest non-negative integer x for which x repititions of the
  group operation on b yields a.
  - Additively: a = x * b
  - Multiplicatively: a = b^x

The discrete logarithm only exists when a is an element of the group
generated by b. In cyclic groups, the discrete logarithm is well defined
for any generator as b, regardless of a.

Cyclic groups have a number of interesting properties:
1) Every cyclic group is abelian
```
Let G be a cyclic group with generator g. 
Then every element in G can be written as g^k for some integer k.

Take any two elements a, b ∈ G:
- a = g^m for some integer m
- b = g^n for some integer n

Then:
- a · b = g^m · g^n = g^(m+n)
- b · a = g^n · g^m = g^(n+m)

Since m + n = n + m (integer addition is commutative), we have:
a · b = g^(m+n) = g^(n+m) = b · a
```

2) For a given order n, all cyclic groups of order n are isomorphic
  We speak of "the" cyclic group of order n as Cn

```
For example, take two order 4 groups:
G1 = { 1, 2, 3, 4 } / Generator = 1 / Operation = mod 4
G2 = { 1, -1, i, -i } / Generator = i / Operation complex multiplication

Isomorphism:
0 <->  1
1 <->  i
2 <-> -1
3 <-> -i

(1 + 3) % 4 = 0 
i * -i = 1 -> 0
```

The groups have the same structure, just different values.

3) For a finite n, every cyclic group of order n is isomorphic to 
  integer addition modulo n
```
Let G = ⟨g⟩ be any cyclic group with geneartor g
      = { 1, g, g^2, g^3 ... g^n-1 }

Mapping function: G -> ℤ/ℤp
m(g^k) = k (mod n)

For example: n = 4

G = { 1, g, g^2, g^3 }
ℤ/ℤp = { 0, 1, 2, 3 }

1:   0 (mod 4) = 0
g:   1 (mod 4) = 1
g^2: 2 (mod 4) = 2
g^3: 3 (mod 4) = 3

g^2 * g^3 = g^4 = g
2 + 3 (mode 4) = 1

- Operation preserving: identity and operations are preserved
- Bijective: unique mapping of elements of the set
```

4) For an infinite n, every cyclic group is isomorphic to integer 
  addition Cinf ≅ (ℤ, +)

We care about the direct product of groups:
- G1 = (X1, +)
- G2 = (X2, x)

G1 x G2 = cartesian product of X1 and X2 with the operation applied
defined as: (a, b) + (c, d) = (a + c, b + d).

```
G₁ = ℤ₃ = {0, 1, 2} and G₂ = ℤ₂ = {0, 1}

G₁ × G₂ = {(0,0), (0,1), (1,0), (1,1), (2,0), (2,1)}

Sample operations:
- (1,0) + (2,1) = (1+2, 0+1)
- (2,1) + (1,0) = (2+1, 1+0)
```

The fundamental theorem of finite abelian groups:
- Every finite aabelian group is isomorphic to the direct product of
  one or more cyclic groups of prime-order power.
  - Generally when working in a group whose order can be factored into
    multiple prime powers, it's generally possible to switch to a 
    subgroup of each size, solve the problem in those, then switch back

Prime-ordered groups:
- Order is a prime number
- Always cyclic and abelian
- Isomorphic to Cp
- Every element is the generator (except identity)
  - The discrete logarithm can use any non-identity element as a base.
- Since subgroup order divides original order, there are only trivial
  subgroups (with one element)

Fields:
- Mathematical structures where most of our traditional techniques
  for solving equations work
  - Things like rational numbers and real numbers are common examples
- Order: number of elements in the field (integer or inf)
- Characteristic: number of times i has to be added to itself to obtain
  0, otherwise characteristic is zero.

A field consists of the following:
- A set X
- Two binary operators, + and x (unlike groups, which only have one)
- Required properties:
  - (X, +) forms an abelian group, typically written additively
  - (X \ {0}, x) is an ableian group, typically written multiplicativly
  - x is distributed over +: a(b + c) = ab + bc
  - 0x = 0 for all x

The order of a finite field is always a prime power: q = p^m
- For some prime number p
- For m > 0
- Every non-zero element in every finite field can be written as the
  power of some fixed constant of the field.

TODO: resume on 1.4 Isomorphisms

## Reading 5

- One way function: an invertible function that is easy to compute,
  whose inverse is difficult to compute
- Trapdoor: a piece of auxillary information that allows the inverse
  to be easily computed.

We have not proven the existence of one way functions:
- This requires solving P = NP along the way
- Security of cryptosystems relies on the assumptions we make about the
  difficulty of inverting our chosen functions.

Discrete Logarithm Problem:
- 𝔽p: a finite field with a prime number of elements 
  - We inter-changeably use 𝔽p and ℤ/pℤ, using equality in 𝔽p and
    congruence in ℤ/pℤ
  - P is a large prime
- We know that there exists a primitive element g, such that every
  non-zero element of 𝔽p is equal to some power of g:
  - g^p-1 = 1 (FLT, given that p does not divide g, because g < p?)
  - 2, g, g^2 ... g^p-2 ∈ 𝔽p*

Let g be the primitive root for 𝔽p and h be a non-zero element of 𝔽p.
The discrete logarithm problem is the problem of finding an exponent x
such that g^x = h (mod p).

x = the discrete logarithm of h to the base g, log-g(h).

If there is one solution, then there are infinitely many, because
FLT tells us that g^p-1 ≡ 1 (mod p):
```
let g^x = h

We know by FLT that: g^p-1 ≡ 1 (mod p)
let x = x + k(p-1)

g^(x - k(p-1))
= (g^x) * (g^p-1)^k
= h * (1)^k
= h
```

Diffie-Hellman key exchange:
- Establish a shared key in the presence of an adversary that can see
  all of your communication.
- Agree on two public parameters:
  - large prime p
  - nonzero integer g modulo p
- Pick a secrets a and b respectively, compute a public value and 
  exchange:
  - Alice: A ≡ g^a (mod p)
  - Bob: B ≡ g^b (mod p)
- Combine public values to get secret:
  - A' ≡ B^a (mod p) = (g^b)^a (mod p)
  - B' ≡ A^b (mod p) = (g^a)^b (mod p)
- The Diffie-Hellman-Problem is to solve for g^ab given the values of
  g^a and g^b, this is slightly different to the DLP
  - This is no harder than the DLP.
  - If attacker can solve DLP, they can break this.
  - We do not know whether breaking DHP breaks DLP.

ElGamal Public Key Cryptosystem
- Diffie-Hellman allows exchange of a secret key, not actual exchange of
  secret messages
- Elements we pick:
  - Large prime number, for which DLP is difficult in its finite field
  - A element g mod p of large prime order
  - Choose a secret number a to act as private key
  - Alice calculate pubkey: A ≡ g^a (mod p)
  - Bob chooses ephemeral key k and message m:
    - c1 ≡ g^k (mod p)
    - c2 ≡ mA^k (mod p)
    - Ciphertext = (c1, c2)
  - Alice decrypts (c1, c2) with:
    - x ≡ c1^a (mod p)
    - m = c2 * x-1 (mod p)

Note:
- Ciphertext is expressed in 2 x the lenth of plaintext
  -> This is called 2-to-one expansion

How do we know that ElGamal depends on the DHP?
- Fix prime p and base g for ElGamal encryption
- Suppose attacker has access to an oracle that decrypts arbitrary
  ElGamal ciphertexts using arbitrary ElGamal pubkeys
- The attacker can then use this oracle to solve the DHP

In DHP we have two public values:
- A ≡  g^a (mod p)
- B ≡  g^b (mod p)

Which need to be used to calculate: g^ab (mod p)

If we have an ElGamal oracle, we will send it:
- Prime p
- Base g
- Public key A (which is g^a (mod p))
- Ciphertext (c1, c2)

The oracle will return (c1^a)-1 * c2 (mod p), which decrypts ciphertext.
- Choose c1 = B
- Choose c2 = 1

Then the oracle will return:
(B^a)-1 * 1
(g^b)^a -1

If we take the modular inverse of this, we get g^ab, which means that
we have solved the DHP with this oracle. So if we have an oracle that
can break ElGamal, we have an oracle that can break DHP.

Group Theory:

The characteristics that we are interested in for the group 𝔽p* with
regard to multiplication are:
- It has an identity element (1 * a = a)
- Every a ∈ 𝔽p has an inverse (a * a^-1 = 1)
- Multiplication is associatitive (a *(b * c) = (a * b) * c)
- Multiplication is commutative (a*b = b*a)

The same is true of addition, with an identity element of 0.

-> Groups consist of a set of numbers and a rule denoted ★ which satisfy
these identity, inverse, associatitivity and commutativity laws.
-> They can be finite or infinite (|G| or #G).

Examples of things that aren't groups:
- Integers under multiplication: there aren't integer inverses for most
  numbers.
- Anything under matrix multiplication: order matters, not commutative.

Resume on page 74 (already covered this in the abstract algebra section)
