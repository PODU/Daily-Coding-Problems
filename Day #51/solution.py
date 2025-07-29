# Day 51: Uniform shuffle via Fisher-Yates, using only rand(1..k) and swaps.
# Each of n! permutations equally likely. Time: O(n), Space: O(1).
import random


def rand_k(k):
    """Perfectly random integer in [1, k]."""
    return random.randint(1, k)


def shuffle(deck):
    for i in range(len(deck) - 1, 0, -1):
        j = rand_k(i + 1) - 1  # uniform index in [0, i]
        deck[i], deck[j] = deck[j], deck[i]


if __name__ == "__main__":
    deck = list(range(52))
    shuffle(deck)
    print(" ".join(map(str, deck)))
    print("valid permutation:", sorted(deck) == list(range(52)))
