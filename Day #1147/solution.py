# Day 1147: Locking in a binary tree.
# Node keeps parent + count of locked descendants; lock/unlock walk ancestors. O(h).
class Node:
    def __init__(self, parent=None):
        self.left = None
        self.right = None
        self.parent = parent
        self.locked = False
        self.locked_desc = 0

    def is_locked(self):
        return self.locked

    def _can_lock(self):
        if self.locked or self.locked_desc > 0:
            return False
        a = self.parent
        while a:
            if a.locked:
                return False
            a = a.parent
        return True

    def lock(self):
        if not self._can_lock():
            return False
        self.locked = True
        a = self.parent
        while a:
            a.locked_desc += 1
            a = a.parent
        return True

    def unlock(self):
        if not self.locked:
            return False
        self.locked = False
        a = self.parent
        while a:
            a.locked_desc -= 1
            a = a.parent
        return True


if __name__ == "__main__":
    root = Node()
    l = Node(root)
    r = Node(root)
    ll = Node(l)
    root.left, root.right, l.left = l, r, ll
    print(l.lock())     # True
    print(ll.lock())    # False
    print(root.lock())  # False
    print(l.unlock())   # True
    print(ll.lock())    # True
