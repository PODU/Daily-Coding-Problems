# Day 697: LRU cache with O(1) get/set.
# Approach: OrderedDict (hash map + doubly linked list) ordered by recency.
# get/set O(1) time, O(n) space.
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
        self.d[key] = value
        if len(self.d) > self.cap:
            self.d.popitem(last=False)


if __name__ == "__main__":
    c = LRUCache(2)
    c.set(1, 1); c.set(2, 2)
    print(c.get(1))   # 1
    c.set(3, 3)       # evicts key 2
    print(c.get(2))   # None
    print(c.get(3))   # 3
