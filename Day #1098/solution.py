# Day 1098: Floor and ceiling of x in a BST.
# Walk down the tree updating candidates using BST ordering.
# Time: O(h). Space: O(1).
class Node:
    def __init__(self, val):
        self.val = val
        self.l = None
        self.r = None


def insert(root, v):
    if root is None:
        return Node(v)
    if v < root.val:
        root.l = insert(root.l, v)
    else:
        root.r = insert(root.r, v)
    return root


def floor_ceil(root, x):
    fl, ce = None, None
    cur = root
    while cur:
        if cur.val == x:
            return x, x
        if cur.val < x:
            fl = cur.val
            cur = cur.r
        else:
            ce = cur.val
            cur = cur.l
    return fl, ce


if __name__ == "__main__":
    root = None
    for v in [8, 4, 12, 2, 6, 10, 14]:
        root = insert(root, v)
    f, c = floor_ceil(root, 5)
    print("floor=%s ceil=%s" % (f, c))  # floor=4 ceil=6
