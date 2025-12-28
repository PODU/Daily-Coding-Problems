# Day 811: Shortest unique prefix via trie of char frequency counts.
# Walk each word until count==1. Time O(total chars), Space O(total chars).


def build_trie(words):
    root = {}
    for w in words:
        cur = root
        for c in w:
            node = cur.setdefault(c, {"cnt": 0, "next": {}})
            node["cnt"] += 1
            cur = node["next"]
    return root


def shortest_prefix(root, w):
    cur = root
    pre = []
    for c in w:
        node = cur[c]
        pre.append(c)
        cur = node["next"]
        if node["cnt"] == 1:
            break
    return "".join(pre)


def main():
    words = ["dog", "cat", "apple", "apricot", "fish"]
    root = build_trie(words)
    res = [shortest_prefix(root, w) for w in words]
    print("[" + ", ".join(res) + "]")


if __name__ == "__main__":
    main()
