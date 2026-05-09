# Day 1495: Build a min-heap-ordered Cartesian tree whose in-order traversal is S.
# Approach: monotonic stack; pop nodes > current as its left subtree. Time O(n), Space O(n).


class Node:
    __slots__ = ("val", "left", "right")

    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def build_cartesian(s):
    stack = []
    for x in s:
        cur = Node(x)
        last = None
        while stack and stack[-1].val > x:
            last = stack.pop()
        cur.left = last
        if stack:
            stack[-1].right = cur
        stack.append(cur)
    return stack[0] if stack else None


def inorder(n, out):
    if not n:
        return
    inorder(n.left, out)
    out.append(n.val)
    inorder(n.right, out)


def pretty(n, depth=0):
    if not n:
        return
    pretty(n.right, depth + 1)
    print(" " * (depth * 4) + str(n.val))
    pretty(n.left, depth + 1)


def listing(n):
    if not n:
        return
    if n.left or n.right:
        kids = []
        if n.left:
            kids.append(str(n.left.val))
        if n.right:
            kids.append(str(n.right.val))
        print(f"  {n.val} -> {' '.join(kids)}")
    listing(n.left)
    listing(n.right)


if __name__ == "__main__":
    s = [3, 2, 6, 1, 9]
    root = build_cartesian(s)

    io = []
    inorder(root, io)
    print("In-order traversal:", " ".join(map(str, io)))

    print("Root:", root.val)
    print("Parent -> children:")
    listing(root)

    print("Tree (rotated 90 deg, root at left):")
    pretty(root)
