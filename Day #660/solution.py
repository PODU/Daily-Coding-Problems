# Day 660: Alien dictionary: build edges from first differing char of adjacent words,
# then Kahn's BFS topological sort. Time O(C + V + E), Space O(V + E).

from collections import deque


def alien_order(words):
    graph = {c: set() for w in words for c in w}
    indeg = {c: 0 for c in graph}
    for a, b in zip(words, words[1:]):
        for x, y in zip(a, b):
            if x != y:
                if y not in graph[x]:
                    graph[x].add(y)
                    indeg[y] += 1
                break
    q = deque(sorted(c for c in graph if indeg[c] == 0))
    order = []
    while q:
        c = q.popleft()
        order.append(c)
        for nxt in sorted(graph[c]):
            indeg[nxt] -= 1
            if indeg[nxt] == 0:
                q.append(nxt)
    return order


def main():
    words = ['xww', 'wxyz', 'wxyw', 'ywx', 'ywz']
    order = alien_order(words)
    print("[" + ", ".join("'" + c + "'" for c in order) + "]")


if __name__ == "__main__":
    main()
