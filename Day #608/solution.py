# Day 608: Word Ladder: BFS over words, mutating one letter at a time; track parents to rebuild path.
# Time: O(N * L * 26). Space: O(N * L).
from collections import deque
import string


def word_ladder(start, end, dictionary):
    dictionary = set(dictionary)
    if end not in dictionary:
        return None
    parent = {start: None}
    q = deque([start])
    while q:
        cur = q.popleft()
        if cur == end:
            path = []
            w = end
            while w is not None:
                path.append(w)
                w = parent[w]
            return path[::-1]
        for i in range(len(cur)):
            for c in string.ascii_lowercase:
                if c == cur[i]:
                    continue
                nxt = cur[:i] + c + cur[i + 1:]
                if nxt in dictionary and nxt not in parent:
                    parent[nxt] = cur
                    q.append(nxt)
    return None


def fmt(path):
    if path is None:
        return "null"
    return "[" + ", ".join('"%s"' % w for w in path) + "]"


if __name__ == "__main__":
    print(fmt(word_ladder("dog", "cat", {"dot", "dop", "dat", "cat"})))
    print(fmt(word_ladder("dog", "cat", {"dot", "tod", "dat", "dar"})))
