# Day 1475: Autocomplete. Preprocess the dictionary into a trie; for a query,
# walk to the prefix node then collect all words in its subtree.
# Build O(total chars); query O(len(prefix) + matches). Space O(total chars).

class Trie:
    def __init__(self):
        self.root = {}

    def insert(self, word, order):
        node = self.root
        for ch in word:
            node = node.setdefault(ch, {})
        node['#'] = (word, order)

    def starts_with(self, prefix):
        node = self.root
        for ch in prefix:
            if ch not in node:
                return []
            node = node[ch]
        out = []

        def dfs(n):
            if '#' in n:
                out.append(n['#'])
            for k, child in n.items():
                if k != '#':
                    dfs(child)
        dfs(node)
        out.sort(key=lambda t: t[1])  # preserve dictionary insertion order
        return [w for w, _ in out]


if __name__ == "__main__":
    t = Trie()
    for i, w in enumerate(["dog", "deer", "deal"]):
        t.insert(w, i)
    print(t.starts_with("de"))  # ['deer', 'deal']
