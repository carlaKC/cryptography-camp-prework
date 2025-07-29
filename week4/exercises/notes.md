# Week 4

## Reading 1

Done offline, no notes.

[ ] Come back to proof that if identification protocol is secure,
    FST is secure.

## Reading 2

Schnorr identification scheme:
- G: is a polynomial time algorithm that takes 1^n as input, and outputs
  a description of a cyclic group:
  - q: the order of the group
  - g: generator point
  - ||q|| = n (the multiplicative order of the group is n)
- To generate keys we:
  - Obtain: (G, q, g) 
  - Choose a uniform x ∈ ℤq
  - Pubkey = g^x, Privkey = x
  - Choose a uniform k ∈ ℤq
  - Set I = g^k, which is the initial message in our identification
    scheme
- The verifier sends a uniform challenge r ∈ ℤq
- The prover computes s = [r*x + k mod q]
- The verifier accepts if g^s * y^-r = I

## Exercise 1

AGM:
- Every time the "algebraic adversary" outputs a group element, it
  must also output a list of exponents to show how to calculate
  that group element as a combination of the group elements that
  have been given to the adversary as inputs
- Eg, CDH:
  - It is hard to compute Z = g^xy given g^x and g^y as input
  - In the AGM, the attacker must output:
    - Z and a list of exponents (a,b,c) such that g^a * X^b * Y^c = Z
  - Requiring that the adversary is able to output these exponents
    rules out "non algebraic adversarier" that attack without performing
    group operations
- Using this model makes a weaker proof than those that don't, as we're
  ruling out a smaller group of adversaries
