# Day 536: Reconstruct BST from postorder: scan right-to-left (preorder of root,right,left)
# with an upper-bound recursion. Time O(n), space O(n).
import sys

class Node:
    __slots__ = ("val", "left", "right")
    def __init__(self, v):
        self.val = v; self.left = None; self.right = None

def reconstruct(post):
    idx = [len(post) - 1]
    def build(bound):
        if idx[0] < 0 or post[idx[0]] < bound:
            return None
        root = Node(post[idx[0]]); idx[0] -= 1
        root.right = build(root.val)
        root.left = build(bound)
        return root
    return build(-sys.maxsize)

def preorder(n, out):
    if not n: return
    out.append(n.val); preorder(n.left, out); preorder(n.right, out)

def inorder(n, out):
    if not n: return
    inorder(n.left, out); out.append(n.val); inorder(n.right, out)

if __name__ == "__main__":
    post = [2, 4, 3, 8, 7, 5]
    root = reconstruct(post)
    pre, ino = [], []
    preorder(root, pre); inorder(root, ino)
    print("preorder: " + " ".join(map(str, pre)))
    print("inorder:  " + " ".join(map(str, ino)))
