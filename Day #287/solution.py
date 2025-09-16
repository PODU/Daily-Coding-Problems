# Day 287: Top-k similar website pairs by Jaccard similarity of user sets.
# Build per-site user sets, compute Jaccard for all pairs, sort desc (ties alpha), take k.
# Time: O(S^2 * U), Space: O(S*U).
from itertools import combinations


def top_k_similar(visits, k):
    sites = {}
    for site, user in visits:
        sites.setdefault(site, set()).add(user)
    names = sorted(sites)
    results = []
    for a, b in combinations(names, 2):
        A, B = sites[a], sites[b]
        inter = len(A & B)
        uni = len(A | B)
        sim = inter / uni if uni else 0.0
        results.append((sim, a, b))
    results.sort(key=lambda r: (-r[0], r[1], r[2]))
    return [(a, b) for _, a, b in results[:k]]


if __name__ == "__main__":
    visits = [('a', 1), ('a', 3), ('a', 5), ('b', 2), ('b', 6),
              ('c', 1), ('c', 2), ('c', 3), ('c', 4), ('c', 5),
              ('d', 4), ('d', 5), ('d', 6), ('d', 7),
              ('e', 1), ('e', 3), ('e', 5), ('e', 6)]
    print(top_k_similar(visits, 1))
