# Day 1457: LFU cache: hashmap key->node, hashmap freq->ordered list (DLL), track minFreq. O(1) per op.
# Time: O(1) get/set. Space: O(capacity).

class Node:
    __slots__ = ("key", "val", "freq", "prev", "next")
    def __init__(self, key=0, val=0):
        self.key = key
        self.val = val
        self.freq = 1
        self.prev = None
        self.next = None


class DLL:
    def __init__(self):
        self.head = Node()
        self.tail = Node()
        self.head.next = self.tail
        self.tail.prev = self.head
        self.size = 0

    def add_front(self, node):
        node.prev = self.head
        node.next = self.head.next
        self.head.next.prev = node
        self.head.next = node
        self.size += 1

    def remove(self, node):
        node.prev.next = node.next
        node.next.prev = node.prev
        self.size -= 1

    def remove_last(self):
        if self.size == 0:
            return None
        node = self.tail.prev
        self.remove(node)
        return node


class LFUCache:
    def __init__(self, capacity):
        self.cap = capacity
        self.min_freq = 0
        self.nodes = {}        # key -> Node
        self.freqs = {}        # freq -> DLL

    def _touch(self, node):
        f = node.freq
        self.freqs[f].remove(node)
        if self.freqs[f].size == 0:
            del self.freqs[f]
            if self.min_freq == f:
                self.min_freq += 1
        node.freq += 1
        self.freqs.setdefault(node.freq, DLL()).add_front(node)

    def get(self, key):
        if key not in self.nodes:
            return None
        node = self.nodes[key]
        self._touch(node)
        return node.val

    def set(self, key, value):
        if self.cap <= 0:
            return
        if key in self.nodes:
            node = self.nodes[key]
            node.val = value
            self._touch(node)
            return
        if len(self.nodes) >= self.cap:
            lru = self.freqs[self.min_freq].remove_last()
            del self.nodes[lru.key]
        node = Node(key, value)
        self.nodes[key] = node
        self.freqs.setdefault(1, DLL()).add_front(node)
        self.min_freq = 1


def fmt(x):
    return "null" if x is None else str(x)


def main():
    cache = LFUCache(2)
    cache.set(1, 1)
    cache.set(2, 2)
    print(fmt(cache.get(1)))   # 1
    cache.set(3, 3)            # evicts key 2
    print(fmt(cache.get(2)))   # null
    cache.get(3)               # access key 3 (raises its freq) so key 1 becomes LFU/LRU victim
    cache.set(4, 4)            # evicts key 1
    print(fmt(cache.get(1)))   # null
    print(fmt(cache.get(3)))   # 3
    print(fmt(cache.get(4)))   # 4


if __name__ == "__main__":
    main()
