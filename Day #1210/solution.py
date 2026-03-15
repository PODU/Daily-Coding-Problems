# Day 1210: Floor and ceiling of a target in a BST.
# Single root-to-leaf descent updating best candidates. Time O(h), Space O(1).


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
    fl = ce = None
    while root:
        if root.val == x:
            return x, x
        if root.val < x:
            fl = root.val
            root = root.r
        else:
            ce = root.val
            root = root.l
    return fl, ce


if __name__ == "__main__":
    root = None
    for v in [8, 4, 12, 2, 6, 10, 14]:
        root = insert(root, v)
    fl, ce = floor_ceil(root, 7)
    print(f"floor={fl if fl is not None else None} ceiling={ce if ce is not None else None}")
    # floor=6 ceiling=8
