# Day 1572: Phone keypad letter combinations via iterative Cartesian product. O(prod*len) time, O(output) space.

def letter_combinations(digits, mapping):
    if not digits:
        return []
    res = [""]
    for d in digits:
        res = [pre + c for pre in res for c in mapping[d]]
    return res


if __name__ == "__main__":
    mapping = {2: "abc", 3: "def"}
    digits = "23"
    res = letter_combinations(digits, {str(k): v for k, v in mapping.items()})
    print("[" + ", ".join('"' + w + '"' for w in res) + "]")
