# Day 81: Phone-number letter combinations via iterative cartesian product.
# Time O(prod of letters * len), Space O(output).


def letter_combinations(mapping, digits):
    if not digits:
        return []
    res = [""]
    for d in digits:
        res = [prefix + letter for prefix in res for letter in mapping[d]]
    return res


if __name__ == "__main__":
    mapping = {
        "2": ["a", "b", "c"], "3": ["d", "e", "f"], "4": ["g", "h", "i"],
        "5": ["j", "k", "l"], "6": ["m", "n", "o"], "7": ["p", "q", "r", "s"],
        "8": ["t", "u", "v"], "9": ["w", "x", "y", "z"],
    }
    print(letter_combinations(mapping, "23"))
    # ['ad', 'ae', 'af', 'bd', 'be', 'bf', 'cd', 'ce', 'cf']
