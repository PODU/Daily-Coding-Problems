# Day 1687: BFS level order; last node dequeued is a deepest node. Time O(n), Space O(n).
from collections import deque

class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def deepest_node(root):
    if root is None:
        return None
    q = deque([root])
    last = root
    while q:
        last = q.popleft()
        if last.left:
            q.append(last.left)
        if last.right:
            q.append(last.right)
    return last

if __name__ == "__main__":
    a = Node('a')
    b = Node('b')
    c = Node('c')
    d = Node('d')
    a.left, a.right = b, c
    b.left = d
    print(deepest_node(a).val)
