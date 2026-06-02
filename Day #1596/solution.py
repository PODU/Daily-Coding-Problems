# Day 1596: Approach: Symmetric k-ary tree via is_mirror recursion comparing children mirror-wise.
# Time O(n), Space O(h) recursion.

class Node:
    def __init__(self, val):
        self.val = val
        self.children = []

def is_mirror(a, b):
    if a is None and b is None:
        return True
    if a is None or b is None:
        return False
    if a.val != b.val:
        return False
    if len(a.children) != len(b.children):
        return False
    k = len(a.children)
    for i in range(k):
        if not is_mirror(a.children[i], b.children[k - 1 - i]):
            return False
    return True

def is_symmetric(root):
    if root is None:
        return True
    return is_mirror(root, root)

if __name__ == "__main__":
    root = Node(4)
    c1 = Node(3); c1.children.append(Node(9))
    c2 = Node(5)
    c3 = Node(3); c3.children.append(Node(9))
    root.children = [c1, c2, c3]
    print("true" if is_symmetric(root) else "false")
