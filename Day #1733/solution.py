# Day 1733: LRU cache via dict preserving insertion order (move_to_end / popitem). O(1) per get/set. Space O(capacity).
from collections import OrderedDict


class LRUCache:
    def __init__(self, n):
        self.cap = n
        self.data = OrderedDict()

    def get(self, key):
        if key not in self.data:
            return None
        self.data.move_to_end(key)
        return self.data[key]

    def set(self, key, value):
        if key in self.data:
            self.data.move_to_end(key)
        self.data[key] = value
        if len(self.data) > self.cap:
            self.data.popitem(last=False)  # evict LRU


def print_get(cache, key):
    v = cache.get(key)
    print("null" if v is None else v)


if __name__ == "__main__":
    c = LRUCache(2)
    c.set(1, 1)
    c.set(2, 2)
    print_get(c, 1)   # 1
    c.set(3, 3)       # evicts 2
    print_get(c, 2)   # null
    c.set(4, 4)       # evicts 1
    print_get(c, 1)   # null
    print_get(c, 3)   # 3
    print_get(c, 4)   # 4
