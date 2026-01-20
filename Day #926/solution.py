# Day 926: Post-order DFS: compute each subtree sum, tally counts in a dict, return most frequent.
# Time O(n), Space O(n).
from collections import Counter


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def most_frequent_subtree_sum(root):
    counts = Counter()

    def dfs(node):
        if not node:
            return 0
        s = node.val + dfs(node.left) + dfs(node.right)
        counts[s] += 1
        return s

    dfs(root)
    best = max(counts.values())
    return min(k for k, v in counts.items() if v == best)


if __name__ == "__main__":
    root = Node(5, Node(2), Node(-5))
    print(most_frequent_subtree_sum(root))
