# Day 770: Misere Nim forced-win check.
# If every heap == 1: first player wins iff count of heaps is even.
# Else: first player wins iff XOR of heaps != 0. O(N).
from functools import reduce


def first_player_wins(heaps):
    xor_sum = reduce(lambda a, b: a ^ b, heaps, 0)
    if all(h == 1 for h in heaps):
        return len(heaps) % 2 == 0
    return xor_sum != 0


if __name__ == "__main__":
    print(first_player_wins([3, 4, 5]))  # True
