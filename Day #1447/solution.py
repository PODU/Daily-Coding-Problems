# Day 1447: Does a secret code (6 distinct digits) exist consistent with all
# (guess, score) pairs? Brute force all 6-permutations of 0-9. Time O(P*G*6).
from itertools import permutations
from typing import Dict


def consistent(guesses: Dict[int, int]) -> bool:
    items = [(f"{g:06d}", s) for g, s in guesses.items()]
    for code in permutations(range(10), 6):
        if code[0] == 0:  # six-digit number, no leading zero
            continue
        ok = True
        for guess, score in items:
            match = sum(1 for i in range(6) if code[i] == int(guess[i]))
            if match != score:
                ok = False
                break
        if ok:
            return True
    return False


if __name__ == "__main__":
    e1 = {175286: 2, 293416: 3, 654321: 0}
    e2 = {123456: 4, 345678: 4, 567890: 4}
    print("True" if consistent(e1) else "False")  # True
    print("True" if consistent(e2) else "False")  # False
