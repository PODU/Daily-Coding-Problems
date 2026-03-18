# Day 1226: Cousins in a binary tree: BFS level by level tracking parent; collect same-level
# nodes with a different parent than target. Time O(n), Space O(n).
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
        found = False
        for _ in range(len(q)):
            node, parent = q.popleft()
            level.append((node, parent))
            if node.val == target:
                found = True
                target_parent = parent
            if node.left:
                q.append((node.left, node))
            if node.right:
                q.append((node.right, node))
        if found:
            return [n.val for n, p in level
                    if n.val != target and p is not target_parent]
    return []


if __name__ == "__main__":
    root = Node(1,
                Node(2, Node(4), Node(5)),
                Node(3, None, Node(6)))
    print(cousins(root, 4))
