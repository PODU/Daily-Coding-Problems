# Day 6: XOR linked list using get_pointer/dereference_pointer simulated via id() and a registry.
# each node stores both = id(prev) XOR id(next). add: O(1), get(i): O(i). Space: O(n).
class Node:
    def __init__(self, val):
        self.val = val
        self.both = 0


class XorList:
    def __init__(self):
        self._registry = {}  # id -> node, keeps refs alive (Python has no raw pointers)
        self.head = None
        self.tail = None

    def get_pointer(self, node):
        if node is None:
            return 0
        self._registry[id(node)] = node
        return id(node)

    def dereference(self, addr):
        return self._registry.get(addr)

    def add(self, val):
        node = Node(val)
        addr = self.get_pointer(node)
        if self.head is None:
            self.head = self.tail = node
            return
        self.tail.both ^= addr                      # tail.next = node
        node.both = self.get_pointer(self.tail)     # node.prev = tail
        self.tail = node

    def get(self, index):
        prev_addr = 0
        cur = self.head
        for _ in range(index):
            if cur is None:
                break
            next_addr = cur.both ^ prev_addr
            prev_addr = self.get_pointer(cur)
            cur = self.dereference(next_addr)
        return cur


if __name__ == "__main__":
    l = XorList()
    for v in [10, 20, 30, 40]:
        l.add(v)
    print(" ".join(str(l.get(i).val) for i in range(4)))
