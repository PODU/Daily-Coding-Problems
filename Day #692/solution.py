# Day 692: Autocomplete - return all dictionary strings having s as a prefix.
# Approach: Trie. Insert words O(total chars); query walks prefix then DFS to
# collect matches. Query O(|s| + #matches * len).


class Trie:
    def __init__(self):
        self.root = {}

    def insert(self, word):
        node = self.root
        for c in word:
            node = node.setdefault(c, {})
        node["$"] = True

    def autocomplete(self, s):
        node = self.root
        for c in s:
            if c not in node:
                return []
            node = node[c]
        out = []

        def dfs(n, cur):
            if n.get("$"):
                out.append(cur)
            for c, nx in n.items():
                if c != "$":
                    dfs(nx, cur + c)

        dfs(node, s)
        return out


if __name__ == "__main__":
    t = Trie()
    for w in ["dog", "deer", "deal"]:
        t.insert(w)
    print(sorted(t.autocomplete("de")))   # ['deal', 'deer']
