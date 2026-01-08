# Day 877: Subtree check: for each node of s, test identical-tree with t. Time O(m*n), Space O(h).

class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def same_tree(a, b):
    if a is None and b is None:
        return True
    if a is None or b is None:
        return False
    return a.val == b.val and same_tree(a.left, b.left) and same_tree(a.right, b.right)


def is_subtree(s, t):
    if s is None:
        return False
    if same_tree(s, t):
        return True
    return is_subtree(s.left, t) or is_subtree(s.right, t)


if __name__ == "__main__":
    s = Node(3, Node(4, Node(1), Node(2)), Node(5))
    t = Node(4, Node(1), Node(2))
    print(str(is_subtree(s, t)).lower())
