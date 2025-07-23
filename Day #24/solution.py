# Day 24: Binary tree locking: each node has a parent pointer and locked_descendant_count.
# lock/unlock check ancestors (O(h)) + descendant count, then update ancestors (O(h)).


class Node:
    def __init__(self, name, parent=None):
        self.name = name
        self.parent = parent
        self.left = None
        self.right = None
        self.locked = False
        self.locked_descendant_count = 0

    def is_locked(self):
        return self.locked

    def _can_lock(self):
        if self.locked_descendant_count > 0:
            return False
        cur = self.parent
        while cur:
            if cur.locked:
                return False
            cur = cur.parent
        return True

    def lock(self):
        if self.locked or not self._can_lock():
            return False
        self.locked = True
        cur = self.parent
        while cur:
            cur.locked_descendant_count += 1
            cur = cur.parent
        return True

    def unlock(self):
        if not self.locked:
            return False
        self.locked = False
        cur = self.parent
        while cur:
            cur.locked_descendant_count -= 1
            cur = cur.parent
        return True


if __name__ == "__main__":
    root = Node("root")
    node1 = Node("node1", root)
    node2 = Node("node2", root)
    root.left, root.right = node1, node2
    node3 = Node("node3", node1)
    node4 = Node("node4", node1)
    node1.left, node1.right = node3, node4

    print("node2.lock() = " + str(node2.lock()).lower())
    print("root.lock() = " + str(root.lock()).lower())
    print("node2.unlock() = " + str(node2.unlock()).lower())
    print("root.lock() = " + str(root.lock()).lower())
