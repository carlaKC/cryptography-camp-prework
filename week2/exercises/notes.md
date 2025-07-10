# Week 2

Proof by Reduction:
- If we have an implication: P -> Q, its contrapositive is ¬ Q -> ¬P
- eg: if DLP is hard, schnorr is hard becomes if schnorr is not hard,
  DLP is not hard
- These statements are logically equivalent:

  | P   | Q   | P→Q | ¬P  | ¬Q  | ¬Q→¬P |
  |-----|-----|-----|-----|-----|-------|
  | T   | T   | T   | F   | F   | T     |
  | T   | F   | F   | F   | T   | F     |
  | F   | T   | T   | T   | F   | T     |
  | F   | F   | T   | T   | T   | T     |

Proof by contrapositive:
- Assume that the conclusion is false
- Arguing that this forces assumptions to be false

eg: aiming to prove if X is hard, Y is also hard
- Show that Y has an efficient solution
- Use the efficient solution to prove that X has one as well

We formalize statements we make about proofs in "attack games"
- Model the adversary as a program that interacts with another
  program, the challenger
- We define a game which is parameterized with the variable elements 
  of the scheme.

*Game*: 
Compute the discrete log of a group element where g is a fixed generator.
Take λ to be our key size parameter.

```
DL-A(λ)

x <- sℤ(λ)
X <- g^x
x' <- A(λ, X)
return x' == x
```

We define the advantage an adversary program A has to be:
A Adv-DL(λ) := Pr[DL-A(λ) = true]

When an adversary can win a game with some probability (say, 50% of the
time) define the advantage as needing to be some amount greater than
that probability. This would be expressed by subtracting the change of
winning from the probability function; this is basically the "edge"
that the attacker has.

We say that the assumption holds if this probability becomes negligible
(this can be specified more precisely; but in this case it becomes
exponentially smaller as key size increases).

We give adversaries as much information and control as we can in a game,
so that we're proving the strongest possible case.

*Game*:
Prove that using a simple bitwise XOR is a secure encryption scheme.

```
DistinguishCipher-A(λ)

m1, m2 <- A(λ)
c <- Encrypt(λ, m1, m2)
b <- A(c)

return b = b'

where Encrypt(λ, m1, m2):
k <- ${0,1}     // pick a key
b <- $(1,2}     // pick a message to encrypt
return E(m, mb) // return its ciphertext
```

Given that the attacker has a 50% chance of a correct guess, we define
the advantage that the attacker needs to have as:
`Adv-DistinquishCiper-A(λ) = Pr[DistinguishCipher-A(λ) = true] - 1/2`

We can define this scheme to be secure if for all adversary programs A,
the advantage of A is negligible.

## Probability Basics

We're dealing with finite sets, where there are is a known number of 
possible outcomes Ω, and subsets of this state space are known as
events.

The function Pr[E] takes a set of events as inputs is defined as:
1. Pr[E] >= 0 for all E ⊂ Ω (probability is not negative)
2. Pr[Ω] = 1
3. Pr[E] = for x in E, sum Pr[x] (probability = sum of each event's prob)

The assignment of probability for individual events is called its
probability distribution.
- Uniform distribution: Pr[x] = 1/= for Ω  with N elements

## Exercise 1

In remarkable sheet for the week.

## Exercise 2

In remarkable sheet for the week

## Exercise 3

In remarkable sheet for the week

## Reading 1

The set of all possible outcomes of an experiment is known as the
"sample space".

Any subset of the sample space is known as an event E:
- Set consisting of possible outcomes of the experiment
- If the outcome is contained in E, it has occurred
- E^c: the complement of E = all points in S that are not in E

We can combine probability sets like normal sets:
- Union is in either
- Intersection is in both
- We define "contained" in the same way (like subsets)
- We define an empty set for no overlap
- Commutative, associative and distributive laws apply

We denote the sum of events like a sigma: U (n=1; 10) En is the
union of E1, E2, ... E10. Likewise for intercection, with a n looking
symbol.

De Morgan's laws:
(Union [i=1 -> n] E_i)^c = Intersection [i=1 -> n] E^c_i
-> The complement of the union of elements is equal to the intersection
  of complements.

(Intersection [i=1 -> n] Ei)^c = Union [i=1 -> n] E^c_i
-> The complement of the intersection of elements is equal to the
   union of the complements.

Relative frequency definition?
The probability of an event occurring is the number of times that we
expect the event to occur as experiments tend to infinity:
P(E) = lim n-> inf n(E) / n
-> The number of times we get E in n experiements / total experiments

How do we _actually_ know that this value will converge as n gets large?
- Some argue this is an axiom (you choose to accept it, or not)
- Others argue that it's not a priori evident that this value will always
  converge

The modern approach to probability theory defines simpler axioms then
aims to prove that this value does in fact converge:
- Axiom 1: 0 <= P(E) <= 1
- Axiom 2: P(S) = 1
- Axiom 3: For any sequence of mutually exclusive events E1, E2 (there
  is no intercection between events):
  P(Union [i=1 -> inf] Ei) = Sum (i=1 -> inf] P(Ei)
  "The probability of at least one of these events occurring is just the
   sum of their respective probabilities"

Some propositions we get from these axioms:
- P(E^c) = 1 - P(E)
- If E ⊆ F, then P(E) <= P(F)
- PE ∪ F) = P(E) + P(F) - P(EF)
  -> When we have independent results, P(EF) = 0

Conditional probability: Pr[E1 | E2] := Pr[E1 ∩ E2] / Pr[E2]
- The probability of E1 given that E2 occurs
- Equal to probability that both occur, divided by probability of just
  E2

This implies that: Pr[E1 ∩ E2] * Pr[E2] = Pr[E1 ∩ E2]
- This allows us to compute probabilities by breaking a state space
  up into cases
- If our spaces has a partition is made up into A ∪ B, where A and
  B are mutually exclusive (ie, their intersection is empty)

We can calculate 
Pr[E] = Pr[E ∩ A] + Pr[E ∩ B] 
      = Pr[E | A] * Pr[A] + Pr[E | B] * Pr[B]

For any partition: U [i=1 -> n] Ai:
Pr[E] = Sum [i=1 -> n] Pr[E ∩ Ai] = Sum[i=1 -> n] Pr[E | Ai] * Pr[Ai]

We say that events are independent if Pr[E1 | E2] = Pr[E1]

## Reading 2

For a one time pad, keys, messages and ciphertexts are all L-bit strings:
- Ideally we have a key that's shorter than the messages
- Instead, we'll use a L-bit "seed" and "stretch" it into a longer
  L-bit string used to mask the message.
- We use a deterministic algorithm G that maps l-bit string to L-bit
  strings to increase the size of our key.

Stream Cipher:
- Key space: {0,1}^l
- Message space: {0,1}^L
- Ciphertext space: {0,1}^L
- E(s,m) = G(s) xor m
- D(s,c) = G(s) xor c

If l < L, we cannot achieve perfect security by Shannon's Theory.

If G satisfies an appropriate security property, it can be semantically
secure:
- If the adversary can't tell the difference between G(s) and some
  random r, then they can't tell the difference between a perfect
  security one time pad and a stream cipher.

PRG:
- Maps from a finite seed space S to a finite output space R
  - In the above example: S = {0,1}^l, R = {0,1}^L
- Security: no efficient adversary can effectively tell the difference
  between G(s) and r (computationally indistinguishable)

We define our game as follows:

For b = 0,1

Challenger:
- Compute r ∈ R
- If b == 0:
  s <- $S
  Send adversary r <- G(s)
- If b == 1:
  Send adversary r <- $R

Adversary:
- Receives r from the challenger
- Computes b' = {0,1}
- Wins if b' = b

Security of Stream Ciphers:
- G is a PRG over ({0,1}^l, {0,1}^L)
- Cipher constructed from G is defined over ({0,1}^l, {0,1}^<=L, {0,1}^<=L)
  - s ∈ {0,1}^l
  - m,c ∈ {0,1}^<=L
- E(s,m) = G(s)[len m] xor m
- D(s,c) = G(s)[len m] xor c

If G is a secure PNG, then stream cipher E constructed from G is a
semantically secure cipher.
