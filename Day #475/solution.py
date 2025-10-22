# Day 475: Tree locking with parent pointers + per-node locked_descendant_count.
# lock/unlock are O(h): walk ancestors once to check, once to update counts.


class Node:
    def __init__(self, name):
        self.name = name
        self.parent = None
        self.left = None
        self.right = None
        self.locked = False
        self.locked_descendant_count = 0

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
        if self.locked_descendant_count > 0:   # a descendant is locked
            return False
        if self._any_ancestor_locked():        # an ancestor is locked
            return False
        self.locked = True
        p = self.parent
        while p:
            p.locked_descendant_count += 1
            p = p.parent
        return True

    def unlock(self):
        if not self.locked:
            return False
        self.locked = False
        p = self.parent
        while p:
            p.locked_descendant_count -= 1
            p = p.parent
        return True


def child(p, c, left):
    if left:
        p.left = c
    else:
        p.right = c
    c.parent = p
    return c


def main():
    n1 = Node("node1")
    n2 = Node("node2")
    n3 = Node("node3")
    n4 = Node("node4")
    n5 = Node("node5")
    child(n1, n2, True)
    child(n1, n3, False)
    child(n2, n4, True)
    child(n2, n5, False)

    print("lock(node4): " + str(n4.lock()))      # True
    print("lock(node2): " + str(n2.lock()))      # False (descendant locked)
    print("unlock(node4): " + str(n4.unlock()))  # True
    print("lock(node2): " + str(n2.lock()))      # True
    print("lock(node1): " + str(n1.lock()))      # False (descendant locked)
    print("lock(node5): " + str(n5.lock()))      # False (ancestor locked)
    print("unlock(node2): " + str(n2.unlock()))  # True
    print("lock(node1): " + str(n1.lock()))      # True


if __name__ == "__main__":
    main()
