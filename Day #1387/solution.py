# Day 1387: BST two-sum: in-order into sorted array, then two-pointer scan for pair summing to K. O(n) time, O(n) space.

class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def inorder(root, a):
    if not root:
        return
    inorder(root.left, a)
    a.append(root.val)
    inorder(root.right, a)


def two_sum(root, k):
    a = []
    inorder(root, a)
    i, j = 0, len(a) - 1
    while i < j:
        s = a[i] + a[j]
        if s == k:
            return a[i], a[j]
        if s < k:
            i += 1
        else:
            j -= 1
    return -1, -1


if __name__ == "__main__":
    root = Node(10)
    root.left = Node(5)
    root.right = Node(15)
    root.right.left = Node(11)
    root.right.right = Node(15)
    x, y = two_sum(root, 20)
    print(x, y)
