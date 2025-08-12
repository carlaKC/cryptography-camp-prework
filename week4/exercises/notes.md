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

1. Initial Setup

Information Created:
- Identification experiment generates (pk, sk)
- A chooses random j ∈ {1,...,q} (the "guess")

Information Transfer:
- Identification experiment → A: gives pk
- A → A': gives pk

2. A' Makes Hash Queries

Information Created:
- A' creates query pairs (I_i, m_i) for i = 1,2,...,q

Information Transfer & Processing:
A' → A: H(I_i, m_i)

If i ≠ j:
  A creates: random r ∈ Ω_pk
  A → A': returns r

If i = j (the special case):
  A → Identification Experiment: sends I_j
  Identification Experiment creates: challenge r
  Identification Experiment → A: returns r
  A → A': returns r

3. A' Makes Signature Requests

Information Created:
- A' creates message m to be signed

Information Transfer & Processing:
A' → A: "sign message m"
A → Trans_sk oracle: query (no input)
Trans_sk creates: transcript (I, r, s) from honest protocol execution
Trans_sk → A: returns (I, r, s)
A → A': returns signature (r, s)

4. A' Produces Forgery

Information Created:
- A' creates forgery (r*, s*) on message m*

Information Transfer & Processing:
A' → A: forgery (r*, s*) on m*
A computes: I* := V(pk, r*, s*)
A checks: does (I*, m*) = (I_j, m_j)?

If YES (A guessed correctly):
  A → Identification Experiment: sends s* as response
  Identification Experiment verifies: V(pk, r, s*) ?= I_j

If NO:
  A aborts (guess was wrong)

Key Information Dependencies

- r in step 2 (when i=j) = same r that A will use in identification response
- I_j from step 2 = same I_j that A sent to identification experiment
- s from step 4* = becomes A's identification response
- Success depends on: A''s forgery using the same (I_j, m_j) that A embedded the challenge into

Success Condition

The proof works because when A guesses correctly (prob. 1/q), the information flows create a valid
identification protocol execution where A impersonates the prover using A''s forgery.

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

## Exercise 2

Complete Proof Walkthrough: Schnorr EUF-CMA Security in AGM

Goal

Prove: If discrete log is hard, then Schnorr signatures are EUF-CMA secure (under AGM + ROM).

Setup

- A': Adversary that can break Schnorr EUF-CMA with non-negligible probability
- A: Our reduction algorithm that uses A' to solve discrete log
- Instance: A receives challenge pk = g^x, must find x

Reduction Algorithm A

Phase 1: Initialize A'

A → A': Give pk as public key

Phase 2: Handle A''s Queries

Signature Oracle Queries

When A' requests signature on message mᵢ:
1. A chooses random eᵢ, sᵢ ∈ Zq
2. A computes Rᵢ = g^sᵢ · pk^(-eᵢ)
3. A programs H(Rᵢ, mᵢ) = eᵢ
4. A → A': Return signature (eᵢ, sᵢ)
Why valid: Verification computes I = g^sᵢ · pk^(-eᵢ) = Rᵢ, checks H(Rᵢ, mᵢ) = eᵢ ✓

Hash Oracle Queries

When A' queries H(R̃ⱼ, m̃ⱼ):
1. A receives AGM representation: R̃ⱼ = g^(a₀) · pk^(a₁) · ∏ᵢRᵢ^(aᵢ₊₁)
2. If (R̃ⱼ, m̃ⱼ) was previously programmed, return that value
3. Otherwise, A chooses random ẽⱼ ∈ Zq
4. A → A': Return ẽⱼ

Phase 3: Extract Discrete Log from Forgery

A' outputs forgery (r*, s*) on message m* with AGM representation:
R* = g^(b₀) · pk^(b₁) · ∏ᵢRᵢ^(bᵢ₊₁)

The Key Insight: Two Representations of R*

Representation 1: From Verification

Since (r*, s*) is a valid signature on m*:
I* = g^(s*) · pk^(-r*)
H(I*, m*) = r*

Representation 2: From AGM

A' provided coefficients showing:
R* = g^(b₀) · pk^(b₁) · ∏ᵢRᵢ^(bᵢ₊₁)

But if A' queried H(R*, m*) and got r*, then R = I**:
g^(s*) · pk^(-r*) = g^(b₀) · pk^(b₁) · ∏ᵢRᵢ^(bᵢ₊₁)

Substituting Known Values

We know Rᵢ = g^(sᵢ) · pk^(-eᵢ), so:
g^(s*) · pk^(-r*) = g^(b₀) · pk^(b₁) · ∏ᵢ(g^(sᵢ) · pk^(-eᵢ))^(bᵢ₊₁)

= g^(b₀ + Σᵢsᵢbᵢ₊₁) · pk^(b₁ - Σᵢeᵢbᵢ₊₁)

Extracting the Discrete Log

Comparing exponents:
s* = b₀ + Σᵢsᵢbᵢ₊₁  (mod q)
-r* = b₁ - Σᵢeᵢbᵢ₊₁  (mod q)

If b₁ - Σᵢeᵢbᵢ₊₁ ≠ 0, then:
pk = g^x where x = (s* - b₀ - Σᵢsᵢbᵢ₊₁)/(b₁ - Σᵢeᵢbᵢ₊₁) mod q

Success Analysis

- If A' forges successfully: We get the algebraic representation
- If the denominator is non-zero: We can extract discrete log
- Probability: Analysis shows this happens with non-negligible probability when A' succeeds

Conclusion

If A' can forge Schnorr signatures with non-negligible probability, then A can solve discrete log with
non-negligible probability. By contrapositive: if discrete log is hard, then Schnorr signatures are
EUF-CMA secure.

Key insight: AGM forces A' to reveal how it constructed R*, allowing us to set up algebraic equations
we can solve for the discrete log.

### Exercise 2 - Notation

The thing that's most confusing here

One of the confuxing things

One of the confusing things in the reference proof we're given is that
there are [different ways](https://cryptocamp.website/t/week-4-readings-and-exercises/127/4?u=carla)
to express Scnhorr Signature schemes.


Generate Keys:
sk <= Zp
pk = g^sk

For some message m:
k <- Zp
R = g^k

Challenge:
e = H(R, pk, m)

Standard Scnhorr Signature:
- Return signature and nonce:
  - s = k + e*x
  - (s, R)
- Verify Signature:
  - e = H(R, pk, m)
  - Check: R = g^s * pk^-e
    - This is equivalent to: g^s = R * pk^e

Returning Challenge:
- Return challenge and signature:
  - s = k + e*x
  - Return (e, s)
- Verify signature:
  - Calculate: R' = g^s * pk^-e
  - Check: H(R', pk, m) = e
