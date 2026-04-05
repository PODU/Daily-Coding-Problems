# Day 1307: Fisher-Yates shuffle using a uniform rand(1..k). Each of N! permutations
# equally likely. Time O(N), Space O(1) extra.
import random

random.seed(12345)


def rand_k(k):
    """Uniform random integer in [1, k]."""
    return random.randint(1, k)


def shuffle_deck(deck):
    for i in range(len(deck) - 1, 0, -1):
        j = rand_k(i + 1) - 1          # uniform in [0, i]
        deck[i], deck[j] = deck[j], deck[i]
    return deck


if __name__ == "__main__":
    deck = list(range(1, 53))
    shuffle_deck(deck)
    print(" ".join(map(str, deck)))
