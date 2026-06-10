# Day 1634: Word ladder: BFS over dictionary words (one-letter changes), tracking parents to rebuild shortest path. O(N*L*N) time.
from collections import deque


def differs_by_one(a, b):
    if len(a) != len(b):
        return False
    diff = 0
    for ca, cb in zip(a, b):
        if ca != cb:
            diff += 1
            if diff > 1:
                return False
    return diff == 1


def ladder(start, end, dictionary):
    visited = {start}
    q = deque([start])
    parent = {}
    while q:
        cur = q.popleft()
        if cur == end:
            path = []
            at = cur
            while True:
                path.append(at)
                if at == start:
                    break
                at = parent[at]
            path.reverse()
            return path
        for w in dictionary:
            if w not in visited and differs_by_one(cur, w):
                visited.add(w)
                parent[w] = cur
                q.append(w)
    return None


def fmt(path):
    if path is None:
        return "null"
    return "[" + ", ".join('"' + w + '"' for w in path) + "]"


def main():
    print(fmt(ladder("dog", "cat", ["dot", "dop", "dat", "cat"])))
    print(fmt(ladder("dog", "cat", ["tod", "dat", "dar", "dot"])))


if __name__ == "__main__":
    main()
