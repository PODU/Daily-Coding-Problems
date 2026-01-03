# Day 848: LRU cache with O(1) get/set using OrderedDict (hash map + linked list).
# move_to_end marks recent use; popitem(last=False) evicts LRU. O(1) per op.
from collections import OrderedDict


class LRUCache:
    def __init__(self, n):
        self.cap = n
        self.d = OrderedDict()

    def get(self, key):
        if key not in self.d:
            return None
        self.d.move_to_end(key)
        return self.d[key]

    def set(self, key, value):
        if key in self.d:
            self.d.move_to_end(key)
        elif len(self.d) == self.cap:
            self.d.popitem(last=False)
        self.d[key] = value


if __name__ == "__main__":
    c = LRUCache(2)
    c.set(1, 1)
    c.set(2, 2)
    print(c.get(1))  # 1
    c.set(3, 3)      # evicts 2
    print(c.get(2))  # None
    c.set(4, 4)      # evicts 1
    print(c.get(1))  # None
    print(c.get(3))  # 3
    print(c.get(4))  # 4
