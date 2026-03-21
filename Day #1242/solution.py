# Day 1242: Word ladder: shortest transformation via BFS over words differing by 1 letter.
# Time O(N * L * 26), Space O(N * L).
from collections import deque


def word_ladder(start, end, dictionary):
    words = set(dictionary)
    if end not in words:
        return None
    q = deque([[start]])
    seen = {start}
    while q:
        path = q.popleft()
        word = path[-1]
        if word == end:
            return path
        for i in range(len(word)):
            for c in "abcdefghijklmnopqrstuvwxyz":
                nxt = word[:i] + c + word[i + 1:]
                if nxt in words and nxt not in seen:
                    seen.add(nxt)
                    q.append(path + [nxt])
    return None


if __name__ == "__main__":
    print(word_ladder("dog", "cat", {"dot", "dop", "dat", "cat"}))
    print(word_ladder("dog", "cat", {"dot", "tod", "dat", "dar"}))
