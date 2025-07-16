# Week 3

## Reading 1 - Negligible Functions

- Even if attacks are possible, they're still secure if they're
  practically infeasible
- Brute force: try every possible value for the key of length λ:
  - Making the key long enough can make attacks infeasible
  - We call λ the security parameter of the scheme
  - Difficulty is roughly 2^λ
    - There are 2^λ possible keys, so every bit that you add doubles
      the possible key space
  - Sometimes only 2^λ/2 depending on the scheme
- "n-bit" security means that the best known attack requires 2^n steps

Thinking of the cost of computing n-bits, by 2018 EC2 pricing, computing
2^n steps costs:
- 2^50 = $3.50
- 2^55 = $100
- 2^65 = $130,000
- 2^75 = $130 million
- 2^99 = 2 quadrillion
- 2^128 = nobody knows

This is just the cost of the 2^n clock cycles, not including the steps
to perform the attack. This isn't considering specialized hardware, 
which could be optimized to lower costs for a specific operation.
To draw the line between "feasible" and "infeasible" attacks, we
need to be able to draw a line at which feasible attacks are ruled
out.
- Asymptotic cost: how does the cost of computation scale as the
  security parameter does to infinity?
- Best: exponential = very difficult (2^λ)
- Worst: small power of λ (λ^2, λ^3 etc)

Polynomial time: 
- Definition: if there exists a constant c > 0 such that for all 
  sufficiently long input strings x, the program stops after no more 
  than O(|x|^c) steps.
- Closure property: repeating a polynomial time process a polynomial
  amount of times is still a polynomial time process.

Our goal:
- No polynomial time attack can successfully break security
- We don't worry about exponential attacks that can break security

Large numbers:
- Representing the number N on a computer only requires ~log2(N) bits
  - Each bit we store in binary doubles the number of N values we can
    represent
- This is actually our security parameter!

Negligible Success Probability:
- We also need to consider the probability of an attack being 
  successful - some games you can just guess, and others are highly
  improbable.
- We also use the asymptotic approach where we consider the probability
  of success as the security parameter tends to infinity

Eg: Blind guessing game
- Attacker is just guessing the key
- P(success) with λ bit security = 1/2^λ
- If they make 10 guesses, P(success) = 10 / 2^λ
  - This value still tends to zero as λ grows because the exponent
    grows far faster than any polynomial number of guesses that can
    be taken

A function f is negligible:
- if for every polynomial p, lim λ-> inf p(λ)f(λ) = 0

We only have to consider polynomials of the form p(λ) = λ^c for some
constant c.

Exercises:
4.2
4.3(a) + (b)
4.4
4.5

The adversary is not allowed to be an arbitrary program - we will
require it to be a probabilistic polynomial time (PPT) program:
- This allows our adversaries to have access to randomness
- The steps in any execution of A must be bounded by some polynomial,
  expressed as p(λ)
-> This is practical
-> Ensures a negligible success can't be turned into a non-negligible
  one with brute force.

We'll use an oracle for our next section:
- Everyone has access to provide with input and get an output
- Nobody can see inner workings of the oracle
- Adversary can make polynomial requests to the oracle

Signature Schemes:
- Signer S who has established public key `pk`, privkey `sk`
- Sign a message using the private key `sk`
- Anyone with access to the public key can verify the message's
  integrity and authenticity

Comparison to MACs:
- Message authentication codes are also used to ensure the integrity of
  transmitted messages.
- For MACs, you have to establish a shared secret key (the advantage of
  digital signatures is that you just need a public key, and it can
  be verified by all recipients).
  - Public verification: can be verified with publicly available 
    information.
  - Signatures are transferrable: can be passed around and still
    verified.
  - Non-repudiations: S can't deny having signed a message (vs MACs
    which have a shared key, so either party could have signed).
- Digital signatures are the public-key counterpart of message
  authentication codes.

Definition: A digital signature scheme consists of three probabilistic
polynomial time algorithms:
- Gen: The key generation algorithm that takes as input a security
  parameter 1^n and outputs a pair of keys (pk, sk).
  - They have length of at least n
  - n can be determined by pk or sk
- Sign: takes aas input a private key sk and a message `m` from some
  message space (that may depend on `pk`). Outputs a signature σ.
- Vrfy: takes as input a public key `pk`, a message `m` and a signature
  σ, outputs b=1 if the signature is valid and b=0 otherwise.

It is required that except with negligible probability over (pk, sk)
output by Gen(1^n), it holds that Vrfy(m, Sign(sk, m)) = 1 for every
allowed message `m`.

If there is a function `l` such that for every (`pk`, `sk`) output by
Gen(1^n) the message space is {0,1}^l(n) then we say that the three
functions make up a signature scheme for messages of length `l(n)`.

The assumption that we're able to convey `pk` requires at least one
reliable, authenticated round of communication:
- This is expensive, but a one time setup (unlimited thereafter)

A forgery is defined as:
- For a public key `pk`, generated by `S`
- For message `m`
- There exists a valid signature `σ` that was not created by `S`

Sig-forge:
- Gen(1^n) is run to obtain (`pk`, `sk`)
- Adversary A is given `pk` and access to an oracle `Sign(m)`
- The adversary outputs `m`, `σ`
- Success if `vrfy(pk, m, σ)` = 1

A scheme is existentially unforegable under an adaptive chosen-message
attack (ie, secure) if for all probabilistic polynomial time adversaries
A there is a negligible function: 
`Pr(Sig-forge) == 1 < negl(n)`

TODO: resume on hash and sign

## Reading 3

- There is no simple / reasonable assumptions regarding hash functions
  that is sufficient for proving constructions using them secure
- Options are:
  - Only use schemes that we can prove using reasonable assumptions
  - Rely on schemes that "aren't broken yet"TM
- Middle ground: introduce an idealized model that is not necessarily
  an accurate representation of reality, and use it to prove security
  - This is better than no proof at all, if the model is reasonable
- Random oracle model: treats a cryptographic hash function as truly
  random:
  - We have a hash function H
  - It can only be evaluated by querying an oracle which returns H(x)
  - We do not claim that this actually exists IRL, it just provides
    a framework
- Methodology:
  - Prove a scheme to be secure in the random oracle model
  - Replace the ROM with the actual hash scheme
- If the hash function is "sufficiently good" at emulating the random
  oracle, then the proof in the first step will carry over to the
  real world.
  - There is no theoretical justification for this logic holding across
    the steps
  - It is not clear what "sufficiently good" means
- Use of random oracle demonstrates that the scheme has no obvious
  design flaws, rather than it actually being proven to be secure.

Random oracle:
- Takes a bit string as input, returns a bit string as output
- Honest parties and the adversary can interact with the random oracle
- Providing x as input and getting y as output is classified as
  "querying the oracle on x"
- We assume queries on the oracle to be private; nobody learns x when
  you query the oracle on x
- The black box is consistent, meaning that it will always yield the
  same output for the same input
- Nobody knows the entire function H except the box itself

In the standard model a sceheme is secure if for all PPT adversaries
A the probability of some event is below some threshold, where this
probability is taken over the random choices of the parties running the
scehme and those of the adversary A.

In the random oracle model, the scheme relies on the oracle H so the
probability is taken over the random choice of H ass well as the random
choices of the parties running the scheme.

Properties of the random oracle:
- If x has not been queried to H, then the value of H(x) is uniform
- If A queries x to H, the reduction and see this query and learn x
- The reduction can set the value of H(x) to a value of its choice,
  as long as this value is correctly distributed (uniform)

## ROM Video

So far we've come across a few assumptions in this course:
- Assumptions that relate to groups: DL, DDH, CDH (these are hard)
- Assumptions that relate to hash functions:
  - Preimage resistance
  - Collision resistance

These are all assumptions about *real* things. For example, we assume
that DL is hard in secp256k1 - this is a real curve that exists, and
we make an assumption about it.

Sometimes it's difficult to prove things secure with assumptions about
things that exist.

The Random Oracle Model is a very different type of assumption:
- It doesn't concretely exist like the other assumptions we've seen
  so far.
- It is an idealized model that we replace hash functions with.
- We're no longer proving things in "real" settings, we prove them
  "under then random oracle model"
  - They may be secure in a real setting (they may also not be)
  - The link between the two is fuzzy.

Go back to Exercise 1 / Week 2:
- Goal was to show that if DLP is hard, then forging two schnorr
  signatures on a given public key with the same nonce is hard (DSF)
- We did this by reduction, flipping things around and proving by
  reduction

When we create a reduction from our DSF to DLP, we subtract the hashes
of our two messages to get the private key of our challenger's pubkey:
- These values are a denominator, so if they're zero the top must also
  be zero (otherwise there is no solution)
- We can't just say that the hash is collision resistant, because we're
  using a black box DSF implementation (it can do whatever it wants)
- One option would be to expand our assumptions to include the hash
  function being collision resistant in our starting assumptions, the
  other would be to prove it.

In the random oracle model, we can make these assumptions about the
hash function that we couldn't just make in the previous example.
- Every time anyone wants to compute a hash, we force them to use
  the random oracle (attacker and honest party)
- The adversary is blind to any internal details of the hash function,
  it's not sha256 anymore it's some idealized thing
- When we're running our reduction, we hook up the DSF to the random
  oracle:
  - This connection is under our control (as we're the one creating
    the VM, or whatever)
  - We can now reason about the probability of a collision because we
    know the probability of getting identical values from the random
    oracle (ie, the birthday problem)
  - Extractability: reduction gets to use all of the hashes that the
    DSF has generated
  - Programmability: you can sub out the random oracle provided that it
    is indistinguishable, this is very powerful (and obviously can't
    be done with a real hash function)
