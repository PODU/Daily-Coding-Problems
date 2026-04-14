# Day 1357: XOR linked list simulated with array-backed memory & integer addresses (no real pointers).
# both = prevIdx ^ nextIdx, NULL sentinel = 0. add=O(1), get(i)=O(i) time, O(n) space.

NULL = 0

class XorList:
    def __init__(self):
        self.value = [0]   # index 0 reserved as NULL
        self.both = [0]
        self.head = NULL
        self.tail = NULL

    def add(self, v):
        idx = len(self.value)
        self.value.append(v)
        self.both.append(0)
        if self.head == NULL:
            self.head = self.tail = idx
            return
        self.both[idx] = self.tail ^ NULL          # prev=tail, next=NULL
        self.both[self.tail] ^= idx                # append as next of old tail
        self.tail = idx

    def get(self, index):
        prev, cur = NULL, self.head
        for _ in range(index):
            nxt = self.both[cur] ^ prev
            prev, cur = cur, nxt
        return self.value[cur]


if __name__ == "__main__":
    lst = XorList()
    for x in (10, 20, 30, 40):
        lst.add(x)
    for i in range(4):
        print(lst.get(i))
