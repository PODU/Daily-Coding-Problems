# Day 469: Mastermind: brute force all 6-permutations of digits 0-9 (10P6=151200),
# keep one consistent with every guess score. Time: O(10P6 * G), Space: O(1).
from itertools import permutations


def score(secret, guess):
    return sum(1 for a, b in zip(secret, guess) if a == b)


def consistent(guesses):
    items = [(str(g).zfill(6), s) for g, s in guesses.items()]
    for perm in permutations("0123456789", 6):
        secret = "".join(perm)
        if all(score(secret, g) == s for g, s in items):
            return True
    return False


if __name__ == "__main__":
    ex1 = {175286: 2, 293416: 3, 654321: 0}
    ex2 = {123456: 4, 345678: 4, 567890: 4}
    print("True" if consistent(ex1) else "False")
    print("True" if consistent(ex2) else "False")
