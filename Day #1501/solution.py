# Day 1501: Most frequent subtree sum.
# Approach: post-order DFS, accumulate subtree sums in a Counter, pick max count.
# Time: O(n), Space: O(n).
from collections import Counter


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def most_frequent_subtree_sum(root):
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
    return sorted(k for k, v in freq.items() if v == best)


if __name__ == "__main__":
    root = Node(5, Node(2), Node(-5))
    print(" ".join(map(str, most_frequent_subtree_sum(root))))
