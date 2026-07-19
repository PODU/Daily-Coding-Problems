# Day 1841: Top-k most similar website pairs by Jaccard similarity of their visitor sets.
# Time O(W^2 * U) over W websites; Space O(total pairs).
from collections import defaultdict
from itertools import combinations


def top_k_similar(pairs, k):
    sites = defaultdict(set)
    for w, u in pairs:
        sites[w].add(u)

    scored = []
    for a, b in combinations(sorted(sites), 2):
        sa, sb = sites[a], sites[b]
        inter = len(sa & sb)
        union = len(sa | sb)
        jac = inter / union if union else 0.0
        scored.append((jac, (a, b)))
    scored.sort(key=lambda x: -x[0])
    return [pair for _, pair in scored[:k]]


def main():
    pairs = [
        ("a", 1), ("a", 3), ("a", 5),
        ("b", 2), ("b", 6),
        ("c", 1), ("c", 2), ("c", 3), ("c", 4), ("c", 5),
        ("d", 4), ("d", 5), ("d", 6), ("d", 7),
        ("e", 1), ("e", 3), ("e", 5), ("e", 6),
    ]
    print(top_k_similar(pairs, 1))


if __name__ == "__main__":
    main()
