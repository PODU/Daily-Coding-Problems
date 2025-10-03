# Day 361: Mastermind feasibility.
# Brute-force all 6-position codes with distinct digits; accept if some code
# matches every guess's score. Time O(P*G*6), P=151200, Space O(1).
from itertools import permutations


def feasible(guesses):
    codes = [f"{g:06d}" for g in guesses]
    scores = list(guesses.values())
    for perm in permutations("0123456789", 6):
        code = "".join(perm)
        if all(sum(a == b for a, b in zip(code, c)) == s
               for c, s in zip(codes, scores)):
            return True
    return False


if __name__ == "__main__":
    print(feasible({175286: 2, 293416: 3, 654321: 0}))
    print(feasible({123456: 4, 345678: 4, 567890: 4}))
