# Day 115: Subtree check via recursive structural match. O(|s|*|t|) worst case.
class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def same(a, b):
    if a is None and b is None:
        return True
    if a is None or b is None or a.val != b.val:
        return False
    return same(a.left, b.left) and same(a.right, b.right)


def is_subtree(s, t):
    if t is None:
        return True
    if s is None:
        return False
    if same(s, t):
        return True
    return is_subtree(s.left, t) or is_subtree(s.right, t)


if __name__ == "__main__":
    s = Node(3, Node(4, Node(1), Node(2)), Node(5))
    t = Node(4, Node(1), Node(2))
    u = Node(4, Node(0))
    print(str(is_subtree(s, t)).lower())  # true
    print(str(is_subtree(s, u)).lower())  # false
