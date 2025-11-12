# Day 586: Group users into a set per site; for every site pair compute Jaccard = |A&B|/|A|B|.
# Sort by similarity DESC, tie-break by pair lexicographically; take top k. Time O(P^2 * U).
from itertools import combinations


def top_k_similar(visits, k):
    sites = {}
    for site, user in visits:
        sites.setdefault(site, set()).add(user)
    names = sorted(sites)
    cands = []
    for x, y in combinations(names, 2):
        a, b = sites[x], sites[y]
        inter = len(a & b)
        uni = len(a | b)
        sim = inter / uni if uni else 0.0
        cands.append((sim, x, y))
    # sort by similarity desc, then pair lexicographically asc
    cands.sort(key=lambda c: (-c[0], c[1], c[2]))
    return [(x, y) for _, x, y in cands[:k]]


if __name__ == "__main__":
    visits = [
        ('a', 1), ('a', 3), ('a', 5),
        ('b', 2), ('b', 6),
        ('c', 1), ('c', 2), ('c', 3), ('c', 4), ('c', 5),
        ('d', 4), ('d', 5), ('d', 6), ('d', 7),
        ('e', 1), ('e', 3), ('e', 5), ('e', 6),
    ]
    print(top_k_similar(visits, 1))
