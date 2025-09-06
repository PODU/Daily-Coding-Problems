# Day 223: In-order traversal with O(1) extra space (Morris traversal).
# Approach: thread each node to its in-order predecessor temporarily, remove thread after visiting.
# Time O(n), Space O(1) (no stack/recursion).
from typing import List, Optional


class Node:
    def __init__(self, val: int, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def morris_inorder(root: Optional[Node]) -> List[int]:
    res: List[int] = []
    cur = root
    while cur:
        if cur.left is None:
            res.append(cur.val)
            cur = cur.right
        else:
            pred = cur.left
            while pred.right is not None and pred.right is not cur:
                pred = pred.right
            if pred.right is None:
                pred.right = cur      # create thread
                cur = cur.left
            else:
                pred.right = None     # remove thread
                res.append(cur.val)
                cur = cur.right
    return res


if __name__ == "__main__":
    root = Node(4, Node(2, Node(1), Node(3)), Node(6, Node(5), Node(7)))
    print(morris_inorder(root))  # [1, 2, 3, 4, 5, 6, 7]
