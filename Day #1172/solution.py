# Day 1172: Reconstruct a binary tree from pre-order and in-order traversals.
# Recursion with a hashmap of in-order positions; first pre-order element is the
# root, its in-order index splits left/right subtrees. Time O(N), Space O(N).


class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def reconstruct(preorder, inorder):
    pos = {v: i for i, v in enumerate(inorder)}
    it = iter(preorder)

    def build(lo, hi):
        if lo > hi:
            return None
        root_val = next(it)
        root = Node(root_val)
        m = pos[root_val]
        root.left = build(lo, m - 1)
        root.right = build(m + 1, hi)
        return root

    return build(0, len(inorder) - 1)


def inorder_str(node):
    if not node:
        return ""
    return inorder_str(node.left) + node.val + inorder_str(node.right)


if __name__ == "__main__":
    pre = ['a', 'b', 'd', 'e', 'c', 'f', 'g']
    ino = ['d', 'b', 'e', 'a', 'f', 'c', 'g']
    root = reconstruct(pre, ino)
    assert inorder_str(root) == "".join(ino)   # verifies reconstruction
    print("    a")
    print("   / \\")
    print("  b   c")
    print(" / \\ / \\")
    print("d  e f  g")
