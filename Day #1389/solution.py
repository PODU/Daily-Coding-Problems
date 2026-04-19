# Day 1389: Top-k website pairs by Jaccard similarity over user sets; sort desc, tie-break alpha.
# O(W^2 * U) to compare pairs.
from itertools import combinations
from collections import defaultdict


def top_k_pairs(visits, k):
    users = defaultdict(set)
    for site, user in visits:
        users[site].add(user)

    results = []
    for a, b in combinations(sorted(users), 2):
        inter = len(users[a] & users[b])
        uni = len(users[a] | users[b])
        sim = inter / uni if uni else 0.0
        results.append((sim, a, b))

    results.sort(key=lambda r: (-r[0], r[1], r[2]))
    return [(a, b) for _, a, b in results[:k]]


def main():
    visits = [('a', 1), ('a', 3), ('a', 5), ('b', 2), ('b', 6),
              ('c', 1), ('c', 2), ('c', 3), ('c', 4), ('c', 5),
              ('d', 4), ('d', 5), ('d', 6), ('d', 7),
              ('e', 1), ('e', 3), ('e', 5), ('e', 6)]
    print(top_k_pairs(visits, 1))


if __name__ == "__main__":
    main()
