# Day 912: Cousins: BFS level by level; on the target's level collect nodes whose parent differs. O(n) time, O(n) space.
from collections import deque


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def cousins(root, target):
    if not root:
        return []
    q = deque([(root, None)])  # (node, parent)
    while q:
        level = []
        target_parent = None
        for _ in range(len(q)):
            node, par = q.popleft()
            level.append((node, par))
            if node.val == target:
                target_parent = par
            if node.left:
                q.append((node.left, node))
            if node.right:
                q.append((node.right, node))
        if target_parent is not None:
            return [n.val for n, p in level if p is not target_parent and n.val != target]
    return []


if __name__ == "__main__":
    root = Node(1)
    root.left = Node(2)
    root.right = Node(3)
    root.left.left = Node(4)
    root.left.right = Node(5)
    root.right.right = Node(6)

    print("Cousins of 4:", cousins(root, 4))  # [6]
    print("Cousins of 6:", cousins(root, 6))  # [4, 5]
