# Day 117: BFS level by level, track level with minimum sum. O(n) time, O(w) space.
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
    level, best, best_sum = 0, 0, float("inf")
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
            best_sum, best = s, level
        level += 1
    return best


if __name__ == "__main__":
    root = Node(10, Node(2, Node(-5), Node(1)), Node(3))
    print(min_sum_level(root))  # 2
