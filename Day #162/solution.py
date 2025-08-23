# Day 162: Shortest unique prefix via trie. Each node stores pass-through count;
# the shortest prefix with count 1 is unique. Time O(total chars), Space O(same).


def shortest_unique_prefixes(words):
    trie = {}
    for w in words:
        node = trie
        for c in w:
            node = node.setdefault(c, {"_count": 0})
            node["_count"] += 1

    result = []
    for w in words:
        node = trie
        prefix = []
        for c in w:
            node = node[c]
            prefix.append(c)
            if node["_count"] == 1:
                break
        result.append("".join(prefix))
    return result


if __name__ == "__main__":
    words = ["dog", "cat", "apple", "apricot", "fish"]
    for p in shortest_unique_prefixes(words):
        print(p)
