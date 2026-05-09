# Day 1497: Step words. Dict word w is a step word of s if len(w)==len(s)+1
# and Counter(s) is a subset of Counter(w) (difference exactly one letter).
# Time O(D*L), Space O(1) (26-letter alphabet).
from collections import Counter


def step_words(word, dictionary):
    base = Counter(word)
    res = []
    for w in dictionary:
        if len(w) != len(word) + 1:
            continue
        diff = Counter(w) - base
        if sum(diff.values()) == 1 and len(base - Counter(w)) == 0:
            res.append(w)
    return res


if __name__ == "__main__":
    input_word = "APPLE"
    dictionary = ["APPEAL", "APPLE", "BANANA", "PLEASE", "APPEALS"]
    print(step_words(input_word, dictionary))
