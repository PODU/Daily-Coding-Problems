# Day 1662: O(1) LFU cache: key->node map, freq->ordered dict(by recency), minFreq pointer.
# get/set O(1); Space O(capacity). Evict least-freq, tie -> least-recently-used.
from collections import OrderedDict, defaultdict

class LFU:
    def __init__(self, cap):
        self.cap = cap
        self.min_freq = 0
        self.vals = {}
        self.freqs = {}
        self.lists = defaultdict(OrderedDict)  # freq -> OrderedDict(key->None), first=LRU

    def _touch(self, key):
        f = self.freqs[key]
        del self.lists[f][key]
        if not self.lists[f]:
            del self.lists[f]
            if self.min_freq == f:
                self.min_freq += 1
        nf = f + 1
        self.freqs[key] = nf
        self.lists[nf][key] = None

    def get(self, key):
        if key not in self.vals:
            return None
        self._touch(key)
        return self.vals[key]

    def set(self, key, val):
        if self.cap <= 0:
            return
        if key in self.vals:
            self.vals[key] = val
            self._touch(key)
            return
        if len(self.vals) >= self.cap:
            lru, _ = self.lists[self.min_freq].popitem(last=False)
            if not self.lists[self.min_freq]:
                del self.lists[self.min_freq]
            del self.vals[lru]
            del self.freqs[lru]
        self.vals[key] = val
        self.freqs[key] = 1
        self.lists[1][key] = None
        self.min_freq = 1

def show(v):
    print("null" if v is None else v)

def main():
    c = LFU(2)
    c.set(1, 1); c.set(2, 2)
    show(c.get(1))
    c.set(3, 3)
    show(c.get(2))
    show(c.get(3))
    c.set(4, 4)
    show(c.get(1))
    show(c.get(3))
    show(c.get(4))

main()
