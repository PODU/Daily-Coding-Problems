# Day 949: Autocomplete - return all words having query as a prefix, using a Trie.
# Build O(total chars); query O(|s| + matches). Insertion order preserved.

class Node:
    __slots__ = ("next", "ids")
    def __init__(self):
        self.next = {}
        self.ids = []


class Trie:
    def __init__(self):
        self.root = Node()
        self.words = []

    def insert(self, w):
        idx = len(self.words)
        self.words.append(w)
        cur = self.root
        for c in w:
            if c not in cur.next:
                cur.next[c] = Node()
            cur = cur.next[c]
            cur.ids.append(idx)

    def with_prefix(self, s):
        cur = self.root
        for c in s:
            if c not in cur.next:
                return []
            cur = cur.next[c]
        return [self.words[i] for i in cur.ids]


if __name__ == "__main__":
    t = Trie()
    for w in ["dog", "deer", "deal"]:
        t.insert(w)
    res = t.with_prefix("de")
    print("[" + ", ".join(res) + "]")  # [deer, deal]
