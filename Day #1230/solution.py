# Day 1230: Trie with pass-through counts; for each word walk down until count==1 to get its shortest unique prefix.
# Time: O(total chars), Space: O(total chars).
from typing import List


class Trie:
    def __init__(self):
        self.next = {}
        self.count = 0


def insert(root: Trie, word: str) -> None:
    cur = root
    for c in word:
        if c not in cur.next:
            cur.next[c] = Trie()
        cur = cur.next[c]
        cur.count += 1


def prefix(root: Trie, word: str) -> str:
    cur = root
    p = []
    for c in word:
        cur = cur.next[c]
        p.append(c)
        if cur.count == 1:
            break
    return "".join(p)


def shortest_unique_prefixes(words: List[str]) -> List[str]:
    root = Trie()
    for w in words:
        insert(root, w)
    return [prefix(root, w) for w in words]


if __name__ == "__main__":
    words = ["dog", "cat", "apple", "apricot", "fish"]
    out = shortest_unique_prefixes(words)
    print("[" + ", ".join(out) + "]")
