# Day 226: Alien Dictionary: build precedence graph from adjacent words, topological sort (Kahn's BFS).
# Time: O(total characters), Space: O(unique letters + edges).
from collections import defaultdict, deque


def alien_order(words):
    adj = defaultdict(set)
    indeg = {}
    for w in words:
        for c in w:
            adj[c]
            indeg.setdefault(c, 0)
    for a, b in zip(words, words[1:]):
        for x, y in zip(a, b):
            if x != y:
                if y not in adj[x]:
                    adj[x].add(y)
                    indeg[y] += 1
                break
    q = deque(sorted(c for c in indeg if indeg[c] == 0))
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
    words = ['xww', 'wxyz', 'wxyw', 'ywx', 'ywz']
    print(alien_order(words))
