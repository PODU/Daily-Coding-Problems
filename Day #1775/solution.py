# Day 1775: Mastermind consistency. Brute-force every 6-digit code with distinct
# digits (P(10,6)=151200); a code is valid if it reproduces every guess's score.
# O(P(10,6) * G) time, O(1) extra space.
from itertools import permutations


def consistent(guesses):
    # guesses: dict {int_guess: score}
    parsed = [([int(c) for c in str(g).zfill(6)], s) for g, s in guesses.items()]
    for code in permutations(range(10), 6):
        ok = True
        for digits, score in parsed:
            m = sum(1 for i in range(6) if code[i] == digits[i])
            if m != score:
                ok = False
                break
        if ok:
            return True
    return False


if __name__ == "__main__":
    print(consistent({175286: 2, 293416: 3, 654321: 0}))  # True
    print(consistent({123456: 4, 345678: 4, 567890: 4}))  # False
