# Day 1014: Word ladder: BFS over words, change one letter per step, track predecessors.
# Time: O(N * L * 26), Space: O(N). Returns shortest path or None (null).
from collections import deque
import string


def ladder(start, end, dictionary):
    if start == end:
        return [start]
    words = set(dictionary)
    q = deque([start])
    parent = {start: None}
    visited = {start}

    while q:
        cur = q.popleft()
        for i in range(len(cur)):
            for c in string.ascii_lowercase:
                if c == cur[i]:
                    continue
                nxt = cur[:i] + c + cur[i + 1:]
                if nxt in words and nxt not in visited:
                    visited.add(nxt)
                    parent[nxt] = cur
                    if nxt == end:
                        path = []
                        w = end
                        while w is not None:
                            path.append(w)
                            w = parent[w]
                        return path[::-1]
                    q.append(nxt)
    return None  # no path


def print_path(p):
    if p is None:
        print("null")
    else:
        print("[" + ", ".join(f'"{w}"' for w in p) + "]")


if __name__ == "__main__":
    print_path(ladder("dog", "cat", {"dot", "dop", "dat", "cat"}))  # ["dog", "dot", "dat", "cat"]
    print_path(ladder("dog", "cat", {"dot", "tod", "dat", "dar"}))  # null
