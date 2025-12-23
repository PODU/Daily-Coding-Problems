# Day 790: Step words: a dict word qualifies if len==word.len+1 and word's letter
# multiset is a subset leaving exactly one extra letter. O(dict*wordLen) time, O(1) space.
from collections import Counter


def step_words(word, dictionary):
    base = Counter(word)
    res = []
    for w in dictionary:
        if len(w) != len(word) + 1:
            continue
        diff = Counter(w)
        diff.subtract(base)
        if all(v >= 0 for v in diff.values()) and sum(diff.values()) == 1:
            res.append(w)
    return res


if __name__ == "__main__":
    word = "APPLE"
    dictionary = ["APPEAL", "BANANA", "ORANGE", "GRAPE"]
    print(" ".join(step_words(word, dictionary)))
