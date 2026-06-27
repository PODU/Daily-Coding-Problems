# Day 1727: Level of a binary tree with the minimum node-value sum.
# BFS level-order, track the level whose sum is smallest. O(n) time, O(w) space (max width).
from collections import deque


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def min_sum_level(root):
    if not root:
        return -1
    q = deque([root])
    level = 0
    best_level, best_sum = 0, float("inf")
    while q:
        s = 0
        for _ in range(len(q)):
            n = q.popleft()
            s += n.val
            if n.left:
                q.append(n.left)
            if n.right:
                q.append(n.right)
        if s < best_sum:
            best_sum, best_level = s, level
        level += 1
    return best_level


if __name__ == "__main__":
    root = Node(5, Node(2, Node(-5)), Node(3))
    print("Level with minimum sum:", min_sum_level(root))
