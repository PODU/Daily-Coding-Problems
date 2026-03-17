# Day 1222: Post-order DFS computing subtree sums, count frequencies in a hashmap,
# return sum(s) with max frequency. O(n) time, O(n) space.
from collections import Counter
from typing import List, Optional


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def most_frequent_subtree_sum(root: Optional[Node]) -> List[int]:
    freq = Counter()

    def dfs(node):
        if not node:
            return 0
        s = node.val + dfs(node.left) + dfs(node.right)
        freq[s] += 1
        return s

    dfs(root)
    if not freq:
        return []
    best = max(freq.values())
    return sorted(s for s, c in freq.items() if c == best)


if __name__ == "__main__":
    root = Node(5, Node(2), Node(-5))
    print(" ".join(map(str, most_frequent_subtree_sum(root))))
