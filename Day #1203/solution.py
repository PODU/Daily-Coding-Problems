# Day 1203: Alien dictionary - derive letter order from sorted words.
# Build precedence graph from adjacent word diffs, Kahn topological sort. Time O(total chars), Space O(1) alphabet.
from collections import deque


def alien_order(words):
    adj = {c: set() for w in words for c in w}
    indeg = {c: 0 for c in adj}
    for a, b in zip(words, words[1:]):
        for ca, cb in zip(a, b):
            if ca != cb:
                if cb not in adj[ca]:
                    adj[ca].add(cb)
                    indeg[cb] += 1
                break
    q = deque(c for c in indeg if indeg[c] == 0)
    order = []
    while q:
        c = q.popleft()
        order.append(c)
        for nx in sorted(adj[c]):
            indeg[nx] -= 1
            if indeg[nx] == 0:
                q.append(nx)
    return order


if __name__ == "__main__":
    words = ["xww", "wxyz", "wxyw", "ywx", "ywz"]
    print([c for c in alien_order(words)])  # ['x', 'z', 'w', 'y']
