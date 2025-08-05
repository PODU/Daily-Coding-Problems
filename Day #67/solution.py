# Day 67: LFU cache: key->(value,freq), freq->OrderedDict of keys (LRU within freq), track minFreq. O(1) get/set.
from collections import OrderedDict, defaultdict


class LFUCache:
    def __init__(self, capacity):
        self.cap = capacity
        self.min_freq = 0
        self.values = {}              # key -> value
        self.freqs = {}               # key -> freq
        self.freq_list = defaultdict(OrderedDict)  # freq -> OrderedDict[key] (front = LRU)

    def _touch(self, key):
        f = self.freqs[key]
        del self.freq_list[f][key]
        if not self.freq_list[f]:
            del self.freq_list[f]
            if self.min_freq == f:
                self.min_freq = f + 1
        nf = f + 1
        self.freqs[key] = nf
        self.freq_list[nf][key] = None

    def get(self, key):
        if key not in self.values:
            return None
        self._touch(key)
        return self.values[key]

    def set(self, key, value):
        if self.cap <= 0:
            return
        if key in self.values:
            self.values[key] = value
            self._touch(key)
            return
        if len(self.values) >= self.cap:
            evict, _ = self.freq_list[self.min_freq].popitem(last=False)  # LRU at min freq
            if not self.freq_list[self.min_freq]:
                del self.freq_list[self.min_freq]
            del self.values[evict]
            del self.freqs[evict]
        self.values[key] = value
        self.freqs[key] = 1
        self.freq_list[1][key] = None
        self.min_freq = 1


def show(v):
    print("null" if v is None else v)


if __name__ == "__main__":
    c = LFUCache(2)
    c.set(1, 1)
    c.set(2, 2)
    show(c.get(1))   # 1
    c.set(3, 3)      # evicts key 2
    show(c.get(2))   # null
    show(c.get(3))   # 3
    c.set(4, 4)      # evicts key 1
    show(c.get(1))   # null
    show(c.get(3))   # 3
    show(c.get(4))   # 4
