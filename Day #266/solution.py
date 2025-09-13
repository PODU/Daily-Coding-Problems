# Day 266: Step words. A dict word is a step word of `word` if its length is
# one greater and its letter multiset is a superset of `word`'s (diff = 1).
# Time O(D * L) over dictionary; space O(alphabet).
from collections import Counter


def is_step_word(word, cand):
    if len(cand) != len(word) + 1:
        return False
    diff = Counter(cand.lower())
    diff.subtract(Counter(word.lower()))
    # all counts must be >= 0 and total extra exactly 1
    if any(v < 0 for v in diff.values()):
        return False
    return sum(diff.values()) == 1


def step_words(word, dictionary):
    return [w for w in dictionary if is_step_word(word, w)]


if __name__ == "__main__":
    word = "APPLE"
    dictionary = ["APPEAL", "APPLES", "KELP", "PALE", "APPLE"]
    print(step_words(word, dictionary))
