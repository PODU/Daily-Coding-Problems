# Day 284: Find all cousins of a node (same level, different parent) via BFS.
# Time O(N), Space O(N).
from collections import deque


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def cousins(root, target):
    q = deque([root])
    while q:
        level = []
        target_parent = None
        for _ in range(len(q)):
            n = q.popleft()
            for c in (n.left, n.right):
                if c:
                    level.append(c.val)
                    if c.val == target:
                        target_parent = n
                    q.append(c)
        if target_parent:
            sib = set()
            if target_parent.left:
                sib.add(target_parent.left.val)
            if target_parent.right:
                sib.add(target_parent.right.val)
            return [v for v in level if v not in sib]
    return []


if __name__ == "__main__":
    root = Node(1, Node(2, Node(4), Node(5)), Node(3, None, Node(6)))
    print(cousins(root, 4))  # [6]
