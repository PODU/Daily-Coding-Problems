# Day 1229: Count unival subtrees via post-order DFS; node is unival if both children unival and values match.
# Time: O(n), Space: O(h) recursion.
from typing import Optional


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def count_unival(root: Optional[Node]) -> int:
    count = 0

    # returns True if subtree is unival; mutates count via nonlocal.
    def dfs(n: Optional[Node]) -> bool:
        nonlocal count
        if n is None:
            return True
        l = dfs(n.left)
        r = dfs(n.right)
        if not l or not r:
            return False
        if n.left and n.left.val != n.val:
            return False
        if n.right and n.right.val != n.val:
            return False
        count += 1
        return True

    dfs(root)
    return count


if __name__ == "__main__":
    root = Node(0,
                Node(1),
                Node(0,
                     Node(1, Node(1), Node(1)),
                     Node(0)))
    print(count_unival(root))
