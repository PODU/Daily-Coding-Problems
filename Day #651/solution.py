# Day 651: Validate BST with inclusive bounds: left<=node, right>=node (duplicates allowed both sides).
# Recursive (low,high) bound check. Time O(n), Space O(h). Python3.

class TreeNode:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def is_valid(node, low, high):
    if node is None:
        return True
    if node.val < low or node.val > high:
        return False
    return is_valid(node.left, low, node.val) and \
        is_valid(node.right, node.val, high)


def is_bst(root):
    return is_valid(root, float("-inf"), float("inf"))


def main():
    # Tree A (valid): root=5, left=3(l=2,r=5), right=8(l=8,r=9)
    a = TreeNode(5)
    a.left = TreeNode(3)
    a.right = TreeNode(8)
    a.left.left = TreeNode(2)
    a.left.right = TreeNode(5)
    a.right.left = TreeNode(8)
    a.right.right = TreeNode(9)

    # Tree B (invalid): root=5, left=3, right=4
    b = TreeNode(5)
    b.left = TreeNode(3)
    b.right = TreeNode(4)

    print("true" if is_bst(a) else "false")
    print("true" if is_bst(b) else "false")


if __name__ == "__main__":
    main()
