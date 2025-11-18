# Day 622: Deepest node in a binary tree via BFS level order; last visited node is deepest.
# Time O(N), Space O(N).
from collections import deque


class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def deepest(root):
    if not root:
        return None
    q = deque([root])
    last = root
    while q:
        last = q.popleft()
        if last.left:
            q.append(last.left)
        if last.right:
            q.append(last.right)
    return last.val


if __name__ == "__main__":
    a, b, c, d = Node('a'), Node('b'), Node('c'), Node('d')
    a.left, a.right = b, c
    b.left = d
    print(deepest(a))  # d
