# Day 1561: BFS level-order traversal of a binary tree using a queue. Time O(n), Space O(n).
from collections import deque


class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def main():
    root = Node(1)
    root.left = Node(2)
    root.right = Node(3)
    root.right.left = Node(4)
    root.right.right = Node(5)

    out = []
    q = deque([root])
    while q:
        n = q.popleft()
        out.append(n.val)
        if n.left:
            q.append(n.left)
        if n.right:
            q.append(n.right)
    print(", ".join(str(x) for x in out))


if __name__ == "__main__":
    main()
