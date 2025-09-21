# Day 307: BST floor (largest <= x) and ceiling (smallest >= x). O(h) per query.
class Node:
    def __init__(self, v):
        self.v = v
        self.l = None
        self.r = None


def insert(root, v):
    if root is None:
        return Node(v)
    if v < root.v:
        root.l = insert(root.l, v)
    else:
        root.r = insert(root.r, v)
    return root


def floor_ceil(root, x):
    floor = ceil = None
    while root:
        if root.v == x:
            return root.v, root.v
        if root.v < x:
            floor = root.v
            root = root.r
        else:
            ceil = root.v
            root = root.l
    return floor, ceil


if __name__ == "__main__":
    root = None
    for v in [8, 4, 12, 2, 6, 10, 14]:
        root = insert(root, v)
    fl, ce = floor_ceil(root, 5)
    print("Floor: {}, Ceiling: {}".format(
        fl if fl is not None else "None", ce if ce is not None else "None"))  # Floor: 4, Ceiling: 6
