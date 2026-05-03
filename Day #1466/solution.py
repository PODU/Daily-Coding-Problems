# Day 1466: Sentence similarity (non-transitive). Store synonym pairs both directions in a set; compare word by word.
# Time O(words + pairs), Space O(pairs).

def are_similar(s1, s2, pairs):
    if len(s1) != len(s2):
        return False
    syn = set()
    for a, b in pairs:
        syn.add((a, b))
        syn.add((b, a))
    for w1, w2 in zip(s1, s2):
        if w1 == w2:
            continue
        if (w1, w2) in syn:
            continue
        return False
    return True


def tokenize(s):
    return s.replace(".", "").split()


if __name__ == "__main__":
    synonyms = [("big", "large"), ("eat", "consume")]
    a = tokenize("He wants to eat food.")
    b = tokenize("He wants to consume food.")
    print("True" if are_similar(a, b, synonyms) else "False")
