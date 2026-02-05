# Day 1023: Alien dictionary - order of letters from sorted words.
# Approach: build precedence graph from adjacent words, Kahn's topological sort.
# Time O(total chars + V + E), Space O(V + E).
from collections import deque


def alien_order(words):
    appear = []
    adj = {}
    indeg = {}
    for w in words:
        for c in w:
            if c not in indeg:
                indeg[c] = 0
                appear.append(c)
                adj[c] = set()

    for a, b in zip(words, words[1:]):
        for ca, cb in zip(a, b):
            if ca != cb:
                if cb not in adj[ca]:
                    adj[ca].add(cb)
                    indeg[cb] += 1
                break

    q = deque(c for c in appear if indeg[c] == 0)
    res = []
    while q:
        c = q.popleft()
        res.append(c)
        for nb in sorted(adj[c]):
            indeg[nb] -= 1
            if indeg[nb] == 0:
                q.append(nb)
    return res


if __name__ == "__main__":
    words = ["xww", "wxyz", "wxyw", "ywx", "ywz"]
    print(alien_order(words))
