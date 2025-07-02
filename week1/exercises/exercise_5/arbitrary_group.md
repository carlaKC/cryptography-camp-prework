# Exercise 5

Prompt

(1) Once you have completed Exercise 4, describe how ElGamal can be 
    implemented using an arbitrary abelian group. 
    -> Don't forget to mention how public keys are computed. 

(2) Explain why using Zp with the operation of addition as your 
    abelian group is insecure.

Answers:
(1) Abelian groups all hold the property that every element in the
    group has an inverse element. This is required for ElGamal to
    decrypt the ciphertext (as the pubkey is our privkey raised to a
    power).

(2) The security of ElGamal depends on it being hard to calculate
    the private key from the public key (this is what makes it difficult
    to decrypt a message if you don't have its public key). If we use
    addition, it is not expensive to calculate the public key from the
    private key as both directions (addition and subtraction) are
    computationally cheap. In contrast to the group under multiplication,
    where computation is cheap (via successive squares exponentiation)
    to multiply but expensive for divide (no efficient algorithm known).
    tl;dr: we don't have the "trapdoor" that makes things easy for us
    and difficult for an adversary.
