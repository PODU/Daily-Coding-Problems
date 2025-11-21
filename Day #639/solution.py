# Day 639: Letter combinations of a phone number.
# Approach: iterative Cartesian product over digit->letters map.
# Time: O(4^n * n), Space: O(4^n).
from itertools import product


def letter_combinations(digits, mapping):
    if not digits:
        return []
    groups = [mapping[d] for d in digits]
    return ["".join(combo) for combo in product(*groups)]


if __name__ == "__main__":
    mapping = {
        "2": ["a", "b", "c"], "3": ["d", "e", "f"], "4": ["g", "h", "i"],
        "5": ["j", "k", "l"], "6": ["m", "n", "o"], "7": ["p", "q", "r", "s"],
        "8": ["t", "u", "v"], "9": ["w", "x", "y", "z"],
    }
    print(letter_combinations("23", mapping))
