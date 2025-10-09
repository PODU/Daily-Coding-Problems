# Day 395: Group anagrams: dict keyed by sorted chars -> list, preserving first-seen group order.
# Time O(N*K log K), Space O(N*K).
from typing import List


def group_anagrams(words: List[str]) -> List[List[str]]:
    idx = {}
    groups: List[List[str]] = []
    for w in words:
        key = "".join(sorted(w))
        if key not in idx:
            idx[key] = len(groups)
            groups.append([w])
        else:
            groups[idx[key]].append(w)
    return groups


if __name__ == "__main__":
    data = ["eat", "ate", "apt", "pat", "tea", "now"]
    groups = group_anagrams(data)
    print("[" + ", ".join("[" + ", ".join("'" + w + "'" for w in g) + "]" for g in groups) + "]")
