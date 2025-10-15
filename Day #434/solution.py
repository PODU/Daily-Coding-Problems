# Day 434: BST floor (largest <= n) and ceiling (smallest >= n).
# Single O(h) walk: if node.val < n it is a floor candidate (go right); if node.val > n it is
# a ceiling candidate (go left); equal means both are n. O(h) time, O(1) space.

class Node:
    def __init__(self, v):
        self.val = v
        self.left = None
        self.right = None


def insert(root, v):
    if root is None:
        return Node(v)
    if v < root.val:
        root.left = insert(root.left, v)
    else:
        root.right = insert(root.right, v)
    return root


def floor_ceiling(root, n):
    floor = None
    ceil = None
    cur = root
    while cur:
        if cur.val == n:
            return n, n
        elif cur.val < n:
            floor = cur.val
            cur = cur.right
        else:
            ceil = cur.val
            cur = cur.left
    return floor, ceil


def main():
    root = None
    for v in [8, 4, 12, 2, 6, 10, 14]:
        root = insert(root, v)
    for n in [5, 8, 15, 1]:
        f, c = floor_ceiling(root, n)
        print("n=%d: floor=%s, ceiling=%s" %
              (n, "None" if f is None else f, "None" if c is None else c))


if __name__ == "__main__":
    main()
