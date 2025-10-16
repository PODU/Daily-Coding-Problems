# Day 442: Cartesian tree (min-heap ordered, in-order == S) built with a
# monotonic stack in O(n) time, O(n) space.


class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def build_cartesian(s):
    stack = []
    for v in s:
        node = Node(v)
        last = None
        while stack and stack[-1].val > v:
            last = stack.pop()
        node.left = last
        if stack:
            stack[-1].right = node
        stack.append(node)
    return stack[0] if stack else None


def show(n, prefix="", tag=""):
    if n is None:
        return
    print(prefix + tag + str(n.val))
    show(n.left, prefix + "  ", "L-- ")
    show(n.right, prefix + "  ", "R-- ")


if __name__ == "__main__":
    root = build_cartesian([3, 2, 6, 1, 9])
    show(root)
    # 1
    #   L-- 2
    #     L-- 3
    #     R-- 6
    #   R-- 9
