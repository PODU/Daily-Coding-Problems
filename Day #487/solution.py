# Day 487: Find all cousins of a target node (same level, different parent).
# BFS level by level tracking each node's parent; on the target's level collect nodes
# whose parent differs from the target's parent. Time O(n), Space O(n).
from collections import deque


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def cousins(root, target):
    if not root:
        return []
    q = deque([(root, None)])  # node, parent
    while q:
        level = []
        target_parent = None
        for _ in range(len(q)):
            node, parent = q.popleft()
            level.append((node, parent))
            if node.val == target:
                target_parent = parent
            if node.left:
                q.append((node.left, node))
            if node.right:
                q.append((node.right, node))
        if target_parent is not None:
            return [n.val for n, p in level
                    if p is not target_parent and n.val != target]
    return []


if __name__ == "__main__":
    root = Node(1,
                Node(2, Node(4), Node(5)),
                Node(3, None, Node(6)))
    for v in cousins(root, 4):
        print(v)  # 6
