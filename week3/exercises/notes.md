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

Q: is the caveat for neglibilbe two way?
