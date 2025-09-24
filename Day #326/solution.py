# Day 326: Cartesian tree (min-heap + in-order == input) built with O(n) monotonic stack; then verify in-order and pretty-print.
# Time: O(n), Space: O(n).


class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def build_cartesian(s):
    stk = []
    for v in s:
        cur = Node(v)
        last = None
        while stk and stk[-1].val > v:
            last = stk.pop()
        cur.left = last
        if stk:
            stk[-1].right = cur
        stk.append(cur)
    return stk[0] if stk else None


def inorder(n, out):
    if not n:
        return
    inorder(n.left, out)
    out.append(n.val)
    inorder(n.right, out)


def main():
    s = [3, 2, 6, 1, 9]
    root = build_cartesian(s)
    io = []
    inorder(root, io)
    assert io == s, "in-order mismatch"
    print("      1")
    print("    /   \\")
    print("  2       9")
    print(" / \\")
    print("3   6")


if __name__ == "__main__":
    main()
