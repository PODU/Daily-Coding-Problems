# Day 8: Count unival subtrees: postorder; a subtree is unival if both children are
# unival and match the node's value. Time: O(n), Space: O(h).
class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def count_unival(root):
    count = 0

    def is_unival(n):
        nonlocal count
        if n is None:
            return True
        l = is_unival(n.left)
        r = is_unival(n.right)
        if not l or not r:
            return False
        if n.left and n.left.val != n.val:
            return False
        if n.right and n.right.val != n.val:
            return False
        count += 1
        return True

    is_unival(root)
    return count


if __name__ == "__main__":
    root = Node(0, Node(1), Node(0, Node(1, Node(1), Node(1)), Node(0)))
    print(count_unival(root))
