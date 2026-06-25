# Day 1712: In-order traversal -> sorted array, then two-pointer for pair summing to K. Time O(n), Space O(n).
class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def inorder(root, acc):
    if not root:
        return
    inorder(root.left, acc)
    acc.append(root.val)
    inorder(root.right, acc)


def find_pair(root, k):
    a = []
    inorder(root, a)
    i, j = 0, len(a) - 1
    while i < j:
        s = a[i] + a[j]
        if s == k:
            return a[i], a[j]
        elif s < k:
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
    pair = find_pair(root, 20)
    print(f"{pair[0]} and {pair[1]}")
