# Day 196: Most frequent subtree sum.
# Postorder DFS computing each node's subtree sum, count frequencies in a Counter.
# Time: O(n), Space: O(n).
from collections import Counter


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def most_frequent_subtree_sum(root):
    freq = Counter()

    def dfs(n):
        if not n:
            return 0
        s = n.val + dfs(n.left) + dfs(n.right)
        freq[s] += 1
        return s

    dfs(root)
    return max(freq, key=lambda k: freq[k])


if __name__ == "__main__":
    root = Node(5, Node(2), Node(-5))
    print(most_frequent_subtree_sum(root))  # 2
