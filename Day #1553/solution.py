# Day 1553: Alien dictionary: build edges from first differing chars of adjacent words,
# then Kahn's topological sort. Time O(total chars), Space O(letters + edges).
import heapq


def alien_order(words):
    adj = {c: set() for w in words for c in w}
    indeg = {c: 0 for c in adj}
    for a, b in zip(words, words[1:]):
        n = min(len(a), len(b))
        j = 0
        while j < n:
            if a[j] != b[j]:
                if b[j] not in adj[a[j]]:
                    adj[a[j]].add(b[j])
                    indeg[b[j]] += 1
                break
            j += 1
        else:
            if len(a) > len(b):  # invalid prefix case
                return []
    heap = sorted(c for c in indeg if indeg[c] == 0)
    heapq.heapify(heap)
    order = []
    while heap:
        c = heapq.heappop(heap)
        order.append(c)
        for nx in sorted(adj[c]):
            indeg[nx] -= 1
            if indeg[nx] == 0:
                heapq.heappush(heap, nx)
    if len(order) != len(indeg):
        return []
    return order


if __name__ == "__main__":
    words = ['xww', 'wxyz', 'wxyw', 'ywx', 'ywz']
    order = alien_order(words)
    print("[" + ", ".join("'%s'" % c for c in order) + "]")
