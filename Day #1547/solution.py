# Day 1547: XOR linked list simulated with a list of nodes indexed by integer addresses.
# add appends in O(1); get traverses with next_addr = cur_both XOR prev_addr in O(index). Space O(n).

class XorList:
    def __init__(self):
        self.val = [0]   # address 0 is sentinel/null
        self.both = [0]  # prev_addr XOR next_addr
        self.head = 0
        self.tail = 0

    def add(self, v):
        addr = len(self.val)
        self.val.append(v)
        self.both.append(0)
        if self.head == 0:
            self.head = self.tail = addr
        else:
            self.both[self.tail] ^= addr
            self.both[addr] ^= self.tail
            self.tail = addr

    def get(self, index):
        prev, cur = 0, self.head
        for _ in range(index):
            nxt = self.both[cur] ^ prev
            prev, cur = cur, nxt
        return self.val[cur]


if __name__ == "__main__":
    lst = XorList()
    for v in [10, 20, 30, 40, 50]:
        lst.add(v)
    print(lst.get(0))
    print(lst.get(2))
    print(lst.get(4))
