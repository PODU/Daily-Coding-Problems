# Day 856: Top k similar website pairs.
# Approach: Jaccard similarity (|A∩B| / |A∪B|) over user sets, take top k pairs.
# Time: O(W^2 * U), Space: O(W * U).
from itertools import combinations


def top_k_similar(visits, k):
    sites = {}
    for site, user in visits:
        sites.setdefault(site, set()).add(user)
    scored = []
    for a, b in combinations(sorted(sites), 2):
        A, B = sites[a], sites[b]
        inter = len(A & B)
        uni = len(A | B)
        sim = inter / uni if uni else 0.0
        scored.append((sim, (a, b)))
    scored.sort(key=lambda x: -x[0])
    return [p for _, p in scored[:k]]


if __name__ == "__main__":
    visits = [('a', 1), ('a', 3), ('a', 5),
              ('b', 2), ('b', 6),
              ('c', 1), ('c', 2), ('c', 3), ('c', 4), ('c', 5),
              ('d', 4), ('d', 5), ('d', 6), ('d', 7),
              ('e', 1), ('e', 3), ('e', 5), ('e', 6)]
    print(top_k_similar(visits, 1))
