# Day 1395: LRU cache via OrderedDict (move_to_end + popitem); get/set O(1). O(1) time, O(n) space.
from collections import OrderedDict


class LRUCache:
    def __init__(self, capacity):
        self.cap = capacity
        self.map = OrderedDict()

    def get(self, key):
        if key not in self.map:
            return None
        self.map.move_to_end(key)
        return self.map[key]

    def set(self, key, value):
        if key in self.map:
            self.map.move_to_end(key)
        self.map[key] = value
        if len(self.map) > self.cap:
            self.map.popitem(last=False)  # evict least-recently-used


def main():
    cache = LRUCache(2)
    cache.set(1, 1)
    cache.set(2, 2)
    print(fmt(cache.get(1)))   # 1
    cache.set(3, 3)            # evicts key 2
    print(fmt(cache.get(2)))   # null
    cache.set(4, 4)            # evicts key 1
    print(fmt(cache.get(1)))   # null
    print(fmt(cache.get(3)))   # 3
    print(fmt(cache.get(4)))   # 4


def fmt(v):
    return "null" if v is None else str(v)


if __name__ == "__main__":
    main()
