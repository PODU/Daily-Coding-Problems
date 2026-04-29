# Day 1440: Ternary search tree with insert and search.
# Approach: each node stores a char + left/mid/right; mid advances the word.
# Time: insert/search O(L * log A) avg, Space: O(total chars).


class Node:
    __slots__ = ("c", "is_end", "left", "mid", "right")

    def __init__(self, c):
        self.c = c
        self.is_end = False
        self.left = self.mid = self.right = None


def insert(root, word, i=0):
    if i >= len(word):
        return root
    ch = word[i]
    if root is None:
        root = Node(ch)
    if ch < root.c:
        root.left = insert(root.left, word, i)
    elif ch > root.c:
        root.right = insert(root.right, word, i)
    else:
        if i + 1 == len(word):
            root.is_end = True
        else:
            root.mid = insert(root.mid, word, i + 1)
    return root


def search(root, word, i=0):
    if root is None or i >= len(word):
        return False
    ch = word[i]
    if ch < root.c:
        return search(root.left, word, i)
    if ch > root.c:
        return search(root.right, word, i)
    if i + 1 == len(word):
        return root.is_end
    return search(root.mid, word, i + 1)


if __name__ == "__main__":
    root = None
    for w in ["code", "cob", "be", "ax", "war", "we"]:
        root = insert(root, w)
    print(search(root, "code"))  # True
    print(search(root, "cob"))   # True
    print(search(root, "we"))    # True
    print(search(root, "cod"))   # False
    print(search(root, "cat"))   # False
