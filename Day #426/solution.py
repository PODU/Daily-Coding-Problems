# Day 426: Level of binary tree with minimum sum (levels are 0-indexed; root = level 0).
# BFS level-order, sum each level, track the minimum. Time O(n), Space O(width).
from collections import deque


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def min_level(root):
    if not root:
        return -1, 0
    q = deque([root])
    level = 0
    best_level = 0
    best_sum = None
    while q:
        s = 0
        for _ in range(len(q)):
            n = q.popleft()
            s += n.val
            if n.left:
                q.append(n.left)
            if n.right:
                q.append(n.right)
        if best_sum is None or s < best_sum:
            best_sum = s
            best_level = level
        level += 1
    return best_level, best_sum


if __name__ == "__main__":
    #     1
    #    / \
    #   2   3
    root = Node(1, Node(2), Node(3))
    lvl, s = min_level(root)
    print(f"Level with minimum sum: {lvl} (sum = {s})")
