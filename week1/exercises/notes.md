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

