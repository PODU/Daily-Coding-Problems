# Day 1406: BFS level-order: sum each level, track the level (1-indexed) with min sum.
# Time O(n), Space O(width).
from collections import deque


class Node:
    def __init__(self, val, l=None, r=None):
        self.val, self.l, self.r = val, l, r


def min_sum_level(root):
    if not root:
        return (-1, 0)
    q = deque([root])
    level, best_level, best_sum = 0, 1, float('inf')
    while q:
        sz = len(q)
        level += 1
        s = 0
        for _ in range(sz):
            n = q.popleft()
            s += n.val
            if n.l:
                q.append(n.l)
            if n.r:
                q.append(n.r)
        if s < best_sum:
            best_sum, best_level = s, level
    return (best_level, best_sum)


if __name__ == "__main__":
    root = Node(10, Node(2, Node(4), Node(5)), Node(3))
    lvl, s = min_sum_level(root)
    print(f"Level with minimum sum: {lvl} (sum = {s})")
