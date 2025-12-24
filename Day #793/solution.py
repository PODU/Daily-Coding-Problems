# Day 793: Prune binary tree to a full binary tree: post-order recursion; if a node has
# exactly one child, replace it with its (already pruned) child. O(n) time, O(h) space.
from collections import deque


class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def prune(root):
    if root is None:
        return None
    root.left = prune(root.left)
    root.right = prune(root.right)
    if root.left and not root.right:
        return root.left
    if root.right and not root.left:
        return root.right
    return root


def main():
    nodes = [Node(i) for i in range(8)]
    nodes[0].left, nodes[0].right = nodes[1], nodes[2]
    nodes[1].left = nodes[3]
    nodes[2].right = nodes[4]
    nodes[3].right = nodes[5]
    nodes[4].left, nodes[4].right = nodes[6], nodes[7]

    root = prune(nodes[0])

    q = deque([root])
    while q:
        level = []
        for _ in range(len(q)):
            cur = q.popleft()
            level.append(str(cur.val))
            if cur.left:
                q.append(cur.left)
            if cur.right:
                q.append(cur.right)
        print(" ".join(level))


if __name__ == "__main__":
    main()
