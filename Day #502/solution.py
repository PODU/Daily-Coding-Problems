# Day 502: Height-balanced check via bottom-up recursion returning height, -1 sentinel = unbalanced.
# Time O(n), Space O(h).


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def height(node):
    if node is None:
        return 0
    lh = height(node.left)
    if lh == -1:
        return -1
    rh = height(node.right)
    if rh == -1:
        return -1
    if abs(lh - rh) > 1:
        return -1
    return max(lh, rh) + 1


def is_balanced(root):
    return height(root) != -1


if __name__ == "__main__":
    # Balanced tree
    a = Node(1, Node(2, Node(4)), Node(3))
    # Unbalanced left-leaning chain 1 -> 2 -> 3 -> 4
    b = Node(1, Node(2, Node(3, Node(4))))

    print("true" if is_balanced(a) else "false")
    print("true" if is_balanced(b) else "false")
