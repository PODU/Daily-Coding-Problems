# Day 89: Validate BST via recursive [lo, hi] range check (left<=root<=right allowed).
# Time O(n), Space O(h).


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def is_bst(root, lo=float("-inf"), hi=float("inf")):
    if root is None:
        return True
    if root.val < lo or root.val > hi:
        return False
    return is_bst(root.left, lo, root.val) and is_bst(root.right, root.val, hi)


if __name__ == "__main__":
    a = Node(5, Node(3, Node(2), Node(4)), Node(8))
    print(str(is_bst(a)).lower())  # true

    b = Node(5, Node(3, None, Node(6)), Node(8))  # 6 in left subtree -> invalid
    print(str(is_bst(b)).lower())  # false
