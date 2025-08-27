# Day 179: Reconstruct BST from postorder: iterate right-to-left as (root,right,left) with a lower bound. O(n) time/space.

class Node:
    def __init__(self, v):
        self.val = v
        self.left = None
        self.right = None

class Builder:
    def __init__(self, post):
        self.post = post
        self.idx = len(post) - 1
    def build(self, lower):
        if self.idx < 0 or self.post[self.idx] < lower:
            return None
        val = self.post[self.idx]
        self.idx -= 1
        node = Node(val)
        node.right = self.build(val)
        node.left  = self.build(lower)
        return node

def inorder(n, out):
    if n:
        inorder(n.left, out); out.append(n.val); inorder(n.right, out)

def postorder(n, out):
    if n:
        postorder(n.left, out); postorder(n.right, out); out.append(n.val)

def main():
    post = [2, 4, 3, 8, 7, 5]
    root = Builder(post).build(float('-inf'))
    ino, po = [], []
    inorder(root, ino)
    postorder(root, po)
    print("Inorder:", " ".join(map(str, ino)))
    print("Postorder:", " ".join(map(str, po)))

if __name__ == "__main__":
    main()
