# Day 170: Word ladder via BFS over one-letter transformations. O(N * L * 26) time, O(N) space.
from collections import deque
import string

def ladder(start, end, dictionary):
    if start == end:
        return [start]
    dict_set = set(dictionary)
    q = deque([start])
    parent = {start: None}
    while q:
        cur = q.popleft()
        for i in range(len(cur)):
            for c in string.ascii_lowercase:
                if c == cur[i]:
                    continue
                nxt = cur[:i] + c + cur[i+1:]
                if nxt in dict_set and nxt not in parent:
                    parent[nxt] = cur
                    if nxt == end:
                        path = []
                        s = end
                        while s is not None:
                            path.append(s)
                            s = parent[s]
                        return path[::-1]
                    q.append(nxt)
    return None

def fmt(path):
    if path is None:
        return "null"
    return "[" + ", ".join('"' + w + '"' for w in path) + "]"

def main():
    print(fmt(ladder("dog", "cat", {"dot", "dop", "dat", "cat"})))
    print(fmt(ladder("dog", "cat", {"dot", "tod", "dat", "dar"})))

if __name__ == "__main__":
    main()
