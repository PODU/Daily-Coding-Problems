# Day 1604: BST floor (largest <= x) & ceiling (smallest >= x). Iterative O(h) time, O(1) space.
# Floor: go right recording when val<=x else left. Ceiling: symmetric.

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


def floor_bst(root, x):
    res = None
    while root:
        if root.val == x:
            return x
        if root.val < x:
            res = root.val
            root = root.r
        else:
            root = root.l
    return res


def ceil_bst(root, x):
    res = None
    while root:
        if root.val == x:
            return x
        if root.val > x:
            res = root.val
            root = root.l
        else:
            root = root.r
    return res


def show(v):
    return "None" if v is None else str(v)


def query(root, x):
    print(f"x={x} -> floor {show(floor_bst(root, x))}, ceiling {show(ceil_bst(root, x))}")


if __name__ == "__main__":
    root = None
    for v in [8, 4, 12, 2, 6, 10, 14]:
        root = insert(root, v)
    query(root, 5)   # floor 4, ceiling 6
    query(root, 8)   # floor 8, ceiling 8
    query(root, 15)  # floor 14, ceiling None
    query(root, 1)   # floor None, ceiling 2
