# Day 994: Print binary tree nodes level by level (BFS).
# Use a queue; dequeue a node, emit it, enqueue its children. O(n) time/space.
from collections import deque


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def level_order(root):
    out, q = [], deque([root] if root else [])
    while q:
        node = q.popleft()
        out.append(node.val)
        if node.left:
            q.append(node.left)
        if node.right:
            q.append(node.right)
    return out


if __name__ == "__main__":
    root = Node(1, Node(2), Node(3, Node(4), Node(5)))
    print(", ".join(map(str, level_order(root))))  # 1, 2, 3, 4, 5
