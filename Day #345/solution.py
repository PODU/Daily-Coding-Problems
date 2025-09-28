# Day 345: Sentence Similarity. Direct (non-transitive) synonym pairs via set.
# Time O(P + N) for P pairs and N words. Space O(P).
# Secondary union-find approach (transitive follow-up) included below.


def tokenize(s):
    out = []
    for w in s.split():
        w = w.rstrip(".,!?;:")
        out.append(w)
    return out


def are_similar(synonyms, s1, s2):
    pairs = set()
    for a, b in synonyms:
        pairs.add((a, b))
        pairs.add((b, a))
    w1, w2 = tokenize(s1), tokenize(s2)
    if len(w1) != len(w2):
        return False
    for x, y in zip(w1, w2):
        if x == y or (x, y) in pairs:
            continue
        return False
    return True


# --- Follow-up (transitive): union-find ---
def are_similar_transitive(synonyms, s1, s2):
    parent = {}

    def find(x):
        parent.setdefault(x, x)
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    def union(a, b):
        parent[find(a)] = find(b)

    for a, b in synonyms:
        union(a, b)
    w1, w2 = tokenize(s1), tokenize(s2)
    if len(w1) != len(w2):
        return False
    return all(x == y or find(x) == find(y) for x, y in zip(w1, w2))


def main():
    synonyms = [("big", "large"), ("eat", "consume")]
    s1 = "He wants to eat food."
    s2 = "He wants to consume food."
    print("equivalent" if are_similar(synonyms, s1, s2) else "not equivalent")


if __name__ == "__main__":
    main()
