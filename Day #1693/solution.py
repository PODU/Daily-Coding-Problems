# Day 1693: Ternary Search Tree: node has char + left/mid/right + isEnd. Compare char: <left, >right, ==mid & advance.
# Insert/search: O(L * log A) average where L=key length, A=alphabet size.


class Node:
    __slots__ = ("c", "is_end", "left", "mid", "right")

    def __init__(self, c):
        self.c = c
        self.is_end = False
        self.left = None
        self.mid = None
        self.right = None


def insert(node, s, i):
    ch = s[i]
    if node is None:
        node = Node(ch)
    if ch < node.c:
        node.left = insert(node.left, s, i)
    elif ch > node.c:
        node.right = insert(node.right, s, i)
    elif i + 1 < len(s):
        node.mid = insert(node.mid, s, i + 1)
    else:
        node.is_end = True
    return node


def search(node, s, i):
    if node is None:
        return False
    ch = s[i]
    if ch < node.c:
        return search(node.left, s, i)
    if ch > node.c:
        return search(node.right, s, i)
    if i + 1 == len(s):
        return node.is_end
    return search(node.mid, s, i + 1)


if __name__ == "__main__":
    root = None
    for w in ["code", "cob", "be", "ax", "war", "we"]:
        root = insert(root, w, 0)

    for q in ["code", "cob", "cod", "war", "wa"]:
        print("true" if search(root, q, 0) else "false")
