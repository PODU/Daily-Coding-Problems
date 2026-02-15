# Day 1072: BFS level-order traversal, track sum per level; return 1-indexed level with min sum. O(n) time, O(n) space.
from collections import deque

class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val; self.left = left; self.right = right

def min_sum_level(root):
    if not root:
        return -1, 0
    q = deque([root])
    level, min_level, min_sum = 1, 1, float('inf')
    while q:
        sz = len(q)
        s = sum(q[i].val for i in range(sz))
        for _ in range(sz):
            cur = q.popleft()
            if cur.left:  q.append(cur.left)
            if cur.right: q.append(cur.right)
        if s < min_sum:
            min_sum, min_level = s, level
        level += 1
    return min_level, min_sum

# Tree 1
r1 = Node(1, Node(2, Node(4), Node(5)), Node(3, None, Node(6)))
lvl, sm = min_sum_level(r1)
print(f"Level with min sum: {lvl} (sum={sm})")

# Tree 2
r2 = Node(10, Node(2), Node(3))
lvl, sm = min_sum_level(r2)
print(f"Level with min sum: {lvl} (sum={sm})")
