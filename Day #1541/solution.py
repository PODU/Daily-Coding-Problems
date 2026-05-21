# Day 1541: Fisher-Yates shuffle: uniform random permutation using only swaps.
# rand(k) gives a uniform int in [1,k]; each of N! orderings is equally likely.
# Time O(N), Space O(1).
import random

_rng = random.Random(12345)


def randk(k):
    # uniform random number between 1 and k inclusive
    return _rng.randint(1, k)


def shuffle(deck):
    for i in range(len(deck) - 1, 0, -1):
        j = randk(i + 1) - 1  # index in [0, i]
        deck[i], deck[j] = deck[j], deck[i]
    return deck


if __name__ == "__main__":
    deck = list(range(1, 53))
    print(" ".join(map(str, shuffle(deck))))
