# Day 1597: Reconstruct binary tree from preorder+inorder using inorder index hashmap
# and a moving preorder pointer. Time O(n), Space O(n).
class Node:
    __slots__ = ("val", "left", "right")
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def build(preorder, inorder):
    idx = {v: i for i, v in enumerate(inorder)}
    pre_idx = 0

    def helper(in_l, in_r):
        nonlocal pre_idx
        if in_l > in_r:
            return None
        root_val = preorder[pre_idx]
        pre_idx += 1
        root = Node(root_val)
        mid = idx[root_val]
        root.left = helper(in_l, mid - 1)
        root.right = helper(mid + 1, in_r)
        return root

    return helper(0, len(inorder) - 1)


def preorder_t(n, out):
    if n: out.append(n.val); preorder_t(n.left, out); preorder_t(n.right, out)

def inorder_t(n, out):
    if n: inorder_t(n.left, out); out.append(n.val); inorder_t(n.right, out)

def postorder_t(n, out):
    if n: postorder_t(n.left, out); postorder_t(n.right, out); out.append(n.val)


if __name__ == "__main__":
    pre = ['a', 'b', 'd', 'e', 'c', 'f', 'g']
    ino = ['d', 'b', 'e', 'a', 'f', 'c', 'g']
    root = build(pre, ino)

    po, pr, io = [], [], []
    postorder_t(root, po)
    preorder_t(root, pr)
    inorder_t(root, io)
    print("postorder: " + " ".join(po))
    print("preorder:  " + " ".join(pr))
    print("inorder:   " + " ".join(io))
