# Day 52: LRU cache backed by an OrderedDict. O(1) get/set.
# Time: O(1) per op, Space: O(n).
from collections import OrderedDict


class LRUCache:
    def __init__(self, n):
        self.cap = n
        self.store = OrderedDict()

    def get(self, key):
        if key not in self.store:
            return None
        self.store.move_to_end(key)  # mark most-recently used
        return self.store[key]

    def set(self, key, value):
        if key in self.store:
            self.store.move_to_end(key)
        self.store[key] = value
        if len(self.store) > self.cap:
            self.store.popitem(last=False)  # evict least-recently used


def show(v):
    return "null" if v is None else v


if __name__ == "__main__":
    c = LRUCache(2)
    c.set(1, 1)
    c.set(2, 2)
    print(show(c.get(1)))  # 1
    c.set(3, 3)            # evicts key 2
    print(show(c.get(2)))  # null
    c.set(4, 4)            # evicts key 1
    print(show(c.get(1)))  # null
    print(show(c.get(3)))  # 3
    print(show(c.get(4)))  # 4
