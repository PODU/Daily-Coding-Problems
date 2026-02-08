# Day 1048: Cartesian tree (min-heap ordered, in-order = S) via linear stack on right spine.
# Build O(n); rotated-90 print + verification. Time O(n), Space O(n).


class Node:
    __slots__ = ("val", "left", "right")

    def __init__(self, v):
        self.val = v
        self.left = None
        self.right = None


def build(s):
    st = []
    for v in s:
        cur = Node(v)
        last = None
        while st and st[-1].val > v:
            last = st.pop()
        cur.left = last
        if st:
            st[-1].right = cur
        st.append(cur)
    return st[0] if st else None


def print_rotated(n, depth):
    if n is None:
        return
    print_rotated(n.right, depth + 1)
    print(" " * (depth * 4) + str(n.val))
    print_rotated(n.left, depth + 1)


def inorder(n, out):
    if n is None:
        return
    inorder(n.left, out)
    out.append(n.val)
    inorder(n.right, out)


def min_heap(n):
    if n is None:
        return True
    if n.left and n.left.val <= n.val:
        return False
    if n.right and n.right.val <= n.val:
        return False
    return min_heap(n.left) and min_heap(n.right)


if __name__ == "__main__":
    s = [3, 2, 6, 1, 9]
    root = build(s)
    print("Cartesian tree (rotated 90 deg, root=%d):" % root.val)
    print_rotated(root, 0)
    out = []
    inorder(root, out)
    print("in-order:", " ".join(map(str, out)))
    print("min-heap ordered:", "true" if min_heap(root) else "false")
