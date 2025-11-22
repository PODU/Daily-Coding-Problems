# Day 642: Step words (add one letter + anagram).
# Approach: a dict word qualifies if len == len(word)+1 and its multiset of
# letters minus the input's leaves exactly one extra letter.
# Time: O(D * L), Space: O(L).
from collections import Counter


def is_step(word, cand):
    if len(cand) != len(word) + 1:
        return False
    diff = Counter(cand) - Counter(word)
    return sum(diff.values()) == 1 and len(Counter(word) - Counter(cand)) == 0


def step_words(word, dictionary):
    return [w for w in dictionary if is_step(word, w)]


if __name__ == "__main__":
    word = "APPLE"
    dictionary = ["APPEAL", "APPEAR", "PEAR", "APPLES", "PALE"]
    print(step_words(word, dictionary))  # ['APPEAL', 'APPLES']
