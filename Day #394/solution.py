# Day 394: Root-to-leaf path sum via DFS subtracting node values; leaf checks remainder==0. O(n) time, O(h) space.

class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def has_path_sum(root, k):
    if root is None:
        return False
    if root.left is None and root.right is None:
        return k - root.val == 0
    return has_path_sum(root.left, k - root.val) or has_path_sum(root.right, k - root.val)


if __name__ == "__main__":
    root = Node(8, Node(4, Node(2), Node(6)), Node(13, None, Node(19)))
    print(has_path_sum(root, 18))
