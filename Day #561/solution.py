# Day 561: Sorted array -> height-balanced BST: recurse on mid=(lo+hi)//2 as root.
# Time: O(N), Space: O(N) for nodes + O(log N) recursion.
from collections import deque


class TreeNode:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def build(a, lo, hi):
    if lo > hi:
        return None
    mid = (lo + hi) // 2
    root = TreeNode(a[mid])
    root.left = build(a, lo, mid - 1)
    root.right = build(a, mid + 1, hi)
    return root


def main():
    a = [1, 2, 3, 4, 5, 6, 7]
    root = build(a, 0, len(a) - 1)
    out = []
    q = deque()
    if root:
        q.append(root)
    while q:
        n = q.popleft()
        out.append(str(n.val))
        if n.left:
            q.append(n.left)
        if n.right:
            q.append(n.right)
    print(" ".join(out))


if __name__ == "__main__":
    main()
