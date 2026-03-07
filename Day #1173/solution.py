# Day 1173: Build a min-heap Cartesian tree whose in-order traversal is S.
# Monotonic-stack construction in a single left-to-right pass. Time O(N), Space O(N).


class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def cartesian_tree(s):
    stack = []
    for v in s:
        cur = Node(v)
        last = None
        while stack and stack[-1].val > v:
            last = stack.pop()
        cur.left = last
        if stack:
            stack[-1].right = cur
        stack.append(cur)
    return stack[0] if stack else None


def inorder(node):
    if not node:
        return []
    return inorder(node.left) + [node.val] + inorder(node.right)


if __name__ == "__main__":
    s = [3, 2, 6, 1, 9]
    root = cartesian_tree(s)
    assert inorder(root) == s        # verifies in-order == S
    print("      1")
    print("    /   \\")
    print("  2       9")
    print(" / \\")
    print("3   6")
