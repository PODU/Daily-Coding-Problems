# Day 368: KV store with update/get/max_key(val).
# kv maps key->value; by_val maps value->sorted list of keys (bisect), so max_key
# is the last element. update O(log n), get O(1), max_key O(1).
from bisect import bisect_left, insort


class KVStore:
    def __init__(self):
        self.kv = {}
        self.by_val = {}

    def update(self, key, val):
        if key in self.kv:
            old = self.kv[key]
            lst = self.by_val[old]
            i = bisect_left(lst, key)
            lst.pop(i)
            if not lst:
                del self.by_val[old]
        self.kv[key] = val
        insort(self.by_val.setdefault(val, []), key)

    def get(self, key):
        return self.kv.get(key)

    def max_key(self, val):
        lst = self.by_val.get(val)
        return lst[-1] if lst else None


if __name__ == "__main__":
    kv = KVStore()
    kv.update(1, 1)
    kv.update(2, 1)
    print(kv.max_key(1))  # 2
