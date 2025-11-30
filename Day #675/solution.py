# Day 675: Sentence equivalence under (non-transitive) synonym pairs. For each position,
# words must be equal or a known synonym pair. Time O(W) with a hashed pair set.
import re


def tokens(s):
    return re.findall(r"[a-z0-9]+", s.lower())


def equivalent(synonyms, s1, s2):
    pairs = set()
    for a, b in synonyms:
        pairs.add((a, b))
        pairs.add((b, a))
    w1, w2 = tokens(s1), tokens(s2)
    if len(w1) != len(w2):
        return False
    return all(a == b or (a, b) in pairs for a, b in zip(w1, w2))


if __name__ == "__main__":
    syn = [("big", "large"), ("eat", "consume")]
    print(equivalent(syn, "He wants to eat food.", "He wants to consume food."))  # True
