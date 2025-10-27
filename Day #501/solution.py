# Day 501: Fisher-Yates shuffle using a rand(k) helper returning [1,k]; O(N) time, O(1) extra space.
# Each of the N! permutations is equally likely. Fixed seed -> reproducible output.
# Statistical note: over many runs each position is uniform across all values.
import random

rng = random.Random(12345)


def randk(k):
    # Uniform integer in [1, k] using the provided RNG.
    return rng.randint(1, k)


def shuffle_deck(arr):
    n = len(arr)
    for i in range(n - 1, 0, -1):
        j = randk(i + 1) - 1  # index in [0, i]
        arr[i], arr[j] = arr[j], arr[i]


if __name__ == "__main__":
    deck = list(range(1, 53))  # cards 1..52
    shuffle_deck(deck)
    print(" ".join(map(str, deck)))
