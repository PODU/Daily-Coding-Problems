# Day 1130: Largest BST subtree size via post-order DFS returning (isBST,size,min,max). O(n) time.
import sys

class Node:
    def __init__(self, val):
        self.val = val
        self.l = None
        self.r = None

best = 0

def dfs(n):
    global best
    if n is None:
        return (True, 0, sys.maxsize, -sys.maxsize)
    lb, ls, lmn, lmx = dfs(n.l)
    rb, rs, rmn, rmx = dfs(n.r)
    if lb and rb and n.val > lmx and n.val < rmn:
        sz = ls + rs + 1
        best = max(best, sz)
        return (True, sz, min(n.val, lmn), max(n.val, rmx))
    return (False, 0, 0, 0)

def main():
    root = Node(10)
    root.l = Node(5); root.r = Node(15)
    root.l.l = Node(1); root.l.r = Node(8)
    root.r.r = Node(7)
    dfs(root)
    print(best)

if __name__ == "__main__":
    main()
