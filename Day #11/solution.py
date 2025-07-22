# Day 11: Autocomplete via Trie: insert all words, walk to prefix node, DFS collect.
# Build: O(total chars); query: O(|prefix| + matches). Results in insertion order.
class TrieNode:
    def __init__(self):
        self.ch = {}
        self.order = -1


class Trie:
    def __init__(self):
        self.root = TrieNode()
        self.counter = 0

    def insert(self, w):
        cur = self.root
        for c in w:
            cur = cur.ch.setdefault(c, TrieNode())
        cur.order = self.counter
        self.counter += 1

    def autocomplete(self, prefix):
        cur = self.root
        for c in prefix:
            if c not in cur.ch:
                return []
            cur = cur.ch[c]
        found = []

        def dfs(n, buf):
            if n.order >= 0:
                found.append((n.order, buf))
            for c in sorted(n.ch):
                dfs(n.ch[c], buf + c)

        dfs(cur, prefix)
        found.sort()
        return [w for _, w in found]


if __name__ == "__main__":
    t = Trie()
    for w in ["dog", "deer", "deal"]:
        t.insert(w)
    print(t.autocomplete("de"))
