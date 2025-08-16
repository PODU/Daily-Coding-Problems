# Day 135: Minimum root-to-leaf path sum (with path reconstruction).
# DFS over the tree. O(n) time, O(h) recursion space.


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def min_path(r):
    if not r:
        return (float("inf"), [])
    if not r.left and not r.right:
        return (r.val, [r.val])
    best = (float("inf"), [])
    for c in (r.left, r.right):
        if not c:
            continue
        sub = min_path(c)
        if sub[0] < best[0]:
            best = sub
    return (best[0] + r.val, [r.val] + best[1])


if __name__ == "__main__":
    root = Node(10, Node(5, right=Node(2)), Node(5, right=Node(1, left=Node(-1))))
    total, path = min_path(root)
    print("%d (path %s)" % (total, path))
