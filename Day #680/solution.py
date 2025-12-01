# Day 680: Trie with per-node pass counts; shortest unique prefix = up to first node count==1. Time O(total chars).

def shortest_unique_prefixes(words):
    root = {}
    for w in words:
        node = root
        for c in w:
            child = node.setdefault(c, {"_count": 0})
            child["_count"] += 1
            node = child
    res = []
    for w in words:
        node = root
        pre = []
        for c in w:
            node = node[c]
            pre.append(c)
            if node["_count"] == 1:
                break
        res.append("".join(pre))
    return res


if __name__ == "__main__":
    words = ["dog", "cat", "apple", "apricot", "fish"]
    print(shortest_unique_prefixes(words))
