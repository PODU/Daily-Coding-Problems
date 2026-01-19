# Day 921: XOR linked list via memory-pool simulation: nodes stored in a list, indices act as
# "addresses"; both = prevId XOR nextId (sentinel 0 = null, real nodes start at 1).
# add O(1) with tail tracking, get O(index). O(1) extra per node.


class Node:
    __slots__ = ("value", "both")

    def __init__(self, value):
        self.value = value
        self.both = 0  # prevId XOR nextId


class XorList:
    def __init__(self):
        self.pool = [None]  # index 0 reserved as null sentinel
        self.head = 0
        self.tail = 0

    def add(self, element):
        node = Node(element)
        self.pool.append(node)
        nid = len(self.pool) - 1
        if self.head == 0:
            self.head = self.tail = nid
        else:
            self.pool[self.tail].both ^= nid  # old tail next becomes nid
            node.both = self.tail             # prev = old tail, next = 0
            self.tail = nid

    def get(self, index):
        prev, cur = 0, self.head
        for _ in range(index):
            if cur == 0:
                break
            nxt = self.pool[cur].both ^ prev
            prev, cur = cur, nxt
        if cur == 0:
            raise IndexError("index out of range")
        return self.pool[cur].value


def main():
    lst = XorList()
    for v in (10, 20, 30, 40, 50):
        lst.add(v)
    print(f"get(0) = {lst.get(0)}")
    print(f"get(2) = {lst.get(2)}")
    print(f"get(4) = {lst.get(4)}")


if __name__ == "__main__":
    main()
