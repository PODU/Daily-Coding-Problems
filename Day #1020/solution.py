# Day 1020: BST two-sum: in-order traversal -> sorted list, two-pointer scan.
# O(n) time, O(n) space.
class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def inorder(r, out):
    if not r:
        return
    inorder(r.left, out)
    out.append(r.val)
    inorder(r.right, out)


def find_pair(root, k):
    v = []
    inorder(root, v)
    i, j = 0, len(v) - 1
    while i < j:
        s = v[i] + v[j]
        if s == k:
            return v[i], v[j]
        if s < k:
            i += 1
        else:
            j -= 1
    return None


if __name__ == "__main__":
    root = Node(10)
    root.left = Node(5)
    root.right = Node(15)
    root.right.left = Node(11)
    root.right.right = Node(15)
    a, b = find_pair(root, 20)
    print(f"{a} and {b}")
