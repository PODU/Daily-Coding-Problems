# Day 1077: K-ary tree symmetry: recursively mirror-match children list. O(n) time/space.
class Node:
    def __init__(self, val, children=None):
        self.val = val
        self.children = children or []

def is_mirror(L, R):
    if L is None and R is None: return True
    if L is None or R is None or L.val != R.val: return False
    n = len(L.children)
    if len(R.children) != n: return False
    return all(is_mirror(L.children[i], R.children[n-1-i]) for i in range(n))

def is_symmetric(root):
    if root is None: return True
    n = len(root.children)
    return all(is_mirror(root.children[i], root.children[n-1-i]) for i in range(n // 2))

# Symmetric: 4 -> [3,5,3], first 3 -> [9], last 3 -> [9]
root = Node(4, [Node(3, [Node(9)]), Node(5), Node(3, [Node(9)])])
print(f"Symmetric: {str(is_symmetric(root)).lower()}")

# Asymmetric: 1 -> [2,3]
r2 = Node(1, [Node(2), Node(3)])
print(f"Symmetric: {str(is_symmetric(r2)).lower()}")
