# Day 590: XOR doubly linked list simulated with a memory list; address = index.
# memory[0] is the null sentinel; both = prevIndex XOR nextIndex.
# add: O(1), get(i): O(i). Space O(n).

class Node:
    def __init__(self, value):
        self.value = value
        self.both = 0  # prevIndex XOR nextIndex


class XorList:
    def __init__(self):
        self.memory = [None]  # index 0 = null sentinel
        self.head = 0
        self.tail = 0

    def add(self, element):
        node = Node(element)
        self.memory.append(node)
        idx = len(self.memory) - 1
        if self.head == 0:
            self.head = self.tail = idx
            return
        node.both = self.tail ^ 0
        tail_node = self.memory[self.tail]
        tail_node.both = (tail_node.both ^ 0) ^ idx
        self.tail = idx

    def get(self, index):
        prev = 0
        cur = self.head
        for _ in range(index):
            nxt = prev ^ self.memory[cur].both
            prev = cur
            cur = nxt
        return self.memory[cur].value


if __name__ == "__main__":
    lst = XorList()
    lst.add(10)
    lst.add(20)
    lst.add(30)
    lst.add(40)
    print(lst.get(0))
    print(lst.get(3))
