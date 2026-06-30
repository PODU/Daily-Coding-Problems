# Day 1739: Tree node with parent pointer + locked_descendant_count; lock/unlock walk ancestors O(h).
# is_locked O(1). lock/unlock O(h). Space O(n).


class Node:
    def __init__(self, parent=None):
        self.parent = parent
        self.left = None
        self.right = None
        self.locked = False
        self.locked_descendants = 0

    def is_locked(self):
        return self.locked

    def _any_ancestor_locked(self):
        p = self.parent
        while p:
            if p.locked:
                return True
            p = p.parent
        return False

    def lock(self):
        if self.locked or self.locked_descendants > 0 or self._any_ancestor_locked():
            return False
        self.locked = True
        p = self.parent
        while p:
            p.locked_descendants += 1
            p = p.parent
        return True

    def unlock(self):
        if not self.locked or self.locked_descendants > 0 or self._any_ancestor_locked():
            return False
        self.locked = False
        p = self.parent
        while p:
            p.locked_descendants -= 1
            p = p.parent
        return True


if __name__ == "__main__":
    root = Node()
    root.left = Node(root)
    root.right = Node(root)
    root.left.left = Node(root.left)
    root.left.right = Node(root.left)
    L = root.left
    LL = root.left.left

    print(str(LL.lock()).lower())
    print(str(L.lock()).lower())
    print(str(root.lock()).lower())
    print(str(LL.unlock()).lower())
    print(str(L.lock()).lower())
    print(str(root.lock()).lower())
