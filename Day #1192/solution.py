# Day 1192: Step word: dict word of length len(input)+1 that contains all input letters plus exactly one extra,
# AND is a genuine anagram (rearrangement), not the input with a letter merely appended (no input prefix).
# Compare 26-letter frequency counts: every count >= input's and total diff == 1. O(D * (L + 26)).
from collections import Counter


def step_words(word, dictionary):
    base = Counter(word)
    res = []
    for w in dictionary:
        if len(w) != len(word) + 1:
            continue
        cnt = Counter(w)
        if (all(cnt[ch] >= base[ch] for ch in base)
                and sum(cnt.values()) - sum(base.values()) == 1
                and not w.startswith(word)):
            res.append(w)
    return res


if __name__ == "__main__":
    word = "APPLE"
    dictionary = ["APPEAL", "APPLE", "PEAR", "PALE", "APPEALS", "PAPER", "APPLES", "LAPEL"]
    for w in step_words(word, dictionary):
        print(w)
