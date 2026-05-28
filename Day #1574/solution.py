# Day 1574: Shortest unique prefix via Trie storing char frequency counts. O(total chars) time & space.

class Node:
    __slots__ = ("ch", "cnt")

    def __init__(self):
        self.ch = {}
        self.cnt = 0


def shortest_unique_prefixes(words):
    root = Node()
    for w in words:
        cur = root
        for c in w:
            if c not in cur.ch:
                cur.ch[c] = Node()
            cur = cur.ch[c]
            cur.cnt += 1
    res = []
    for w in words:
        cur = root
        pre = []
        for c in w:
            cur = cur.ch[c]
            pre.append(c)
            if cur.cnt == 1:
                break
        res.append("".join(pre))
    return res


if __name__ == "__main__":
    words = ["dog", "cat", "apple", "apricot", "fish"]
    for p in shortest_unique_prefixes(words):
        print(p)
