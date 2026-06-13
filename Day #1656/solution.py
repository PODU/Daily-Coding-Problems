# Day 1656: Trie autocomplete: insert words, DFS from prefix node in child-insertion order.
# O(total chars) build, O(matches) query; O(total chars) space.
class Node:
    def __init__(self):
        self.children = {}  # dict preserves insertion order
        self.word = None


class Trie:
    def __init__(self):
        self.root = Node()

    def insert(self, w):
        n = self.root
        for c in w:
            if c not in n.children:
                n.children[c] = Node()
            n = n.children[c]
        n.word = w

    def autocomplete(self, s):
        n = self.root
        for c in s:
            if c not in n.children:
                return []
            n = n.children[c]
        out = []
        self._dfs(n, out)
        return out

    def _dfs(self, n, out):
        if n.word is not None:
            out.append(n.word)
        for c in n.children.values():
            self._dfs(c, out)


t = Trie()
for w in ["dog", "deer", "deal"]:
    t.insert(w)
res = t.autocomplete("de")
print("[" + ", ".join(res) + "]")
