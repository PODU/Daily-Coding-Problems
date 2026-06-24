# Day 1705: Cousins: BFS tracking parent & depth; find target's depth+parent, collect nodes at
# same depth with different parent. Time O(n), Space O(n).
from collections import deque

class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

def cousins(root, target):
    q = deque([(root, None)])  # (node, parent)
    while q:
        level = list(q)
        q.clear()
        target_parent = None
        found = False
        for node, par in level:
            if node.val == target:
                target_parent = par
                found = True
            if node.left:
                q.append((node.left, node))
            if node.right:
                q.append((node.right, node))
        if found:
            return [node.val for node, par in level if par is not target_parent]
    return []

def main():
    root = Node(1,
                Node(2, Node(4), Node(5)),
                Node(3, None, Node(6)))
    print(cousins(root, 4))

if __name__ == "__main__":
    main()
