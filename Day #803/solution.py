# Day 803: Does a secret (6 distinct digits) exist matching all guess scores?
# Brute force all 6-digit distinct-digit codes, verify every guess's score.
# Time O(P(10,6) * G), Space O(G).
from itertools import permutations


def feasible(guesses):
    # guesses: dict {guess_int: score}
    gd = [([int(c) for c in str(g).zfill(6)], sc) for g, sc in guesses.items()]
    for code in permutations(range(10), 6):
        if all(sum(a == b for a, b in zip(code, gdig)) == sc for gdig, sc in gd):
            return True
    return False


if __name__ == "__main__":
    print(feasible({175286: 2, 293416: 3, 654321: 0}))  # True
    print(feasible({123456: 4, 345678: 4, 567890: 4}))  # False
