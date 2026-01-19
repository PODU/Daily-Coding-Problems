# Day 922: Locking binary tree: each node tracks lockedDescendants count; lock/unlock check
# ancestors + descendant count. All ops O(h) where h is tree height.


class Node:
    def __init__(self):
        self.parent = None
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
        if self.locked:
            return False
        if self.locked_descendants > 0:
            return False
        if self._any_ancestor_locked():
            return False
        self.locked = True
        p = self.parent
        while p:
            p.locked_descendants += 1
            p = p.parent
        return True

    def unlock(self):
        if not self.locked:
            return False
        self.locked = False
        p = self.parent
        while p:
            p.locked_descendants -= 1
            p = p.parent
        return True


def main():
    root, a, b, c, d = Node(), Node(), Node(), Node(), Node()
    root.left, root.right = a, b
    a.parent = b.parent = root
    a.left, a.right = c, d
    c.parent = d.parent = a

    print(f"lock c (leaf)      -> {c.lock()} (expect True)")
    print(f"lock a (ancestor)  -> {a.lock()} (expect False)")
    print(f"lock root          -> {root.lock()} (expect False)")
    print(f"unlock c           -> {c.unlock()} (expect True)")
    print(f"lock a             -> {a.lock()} (expect True)")
    print(f"lock c (desc lock) -> {c.lock()} (expect False)")
    print(f"unlock a           -> {a.unlock()} (expect True)")


if __name__ == "__main__":
    main()
