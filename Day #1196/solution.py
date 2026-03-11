# Day 1196: All O(1) data structure (LeetCode 432).
# Doubly linked list of count-buckets (each holds a set of keys) + dict key->bucket. All ops O(1); space O(N).

class Bucket:
    __slots__ = ("count", "keys", "prev", "next")
    def __init__(self, count):
        self.count = count
        self.keys = set()
        self.prev = None
        self.next = None


class AllOne:
    def __init__(self):
        self.head = Bucket(float("-inf"))
        self.tail = Bucket(float("inf"))
        self.head.next = self.tail
        self.tail.prev = self.head
        self.key_bucket = {}

    def _insert_after(self, node, count):
        b = Bucket(count)
        b.prev, b.next = node, node.next
        node.next.prev = b
        node.next = b
        return b

    def _remove(self, b):
        b.prev.next = b.next
        b.next.prev = b.prev

    def plus(self, key):
        if key in self.key_bucket:
            cur = self.key_bucket[key]
            nxt = cur.next
            if nxt is self.tail or nxt.count != cur.count + 1:
                nxt = self._insert_after(cur, cur.count + 1)
            nxt.keys.add(key)
            self.key_bucket[key] = nxt
            cur.keys.discard(key)
            if not cur.keys:
                self._remove(cur)
        else:
            first = self.head.next
            if first is self.tail or first.count != 1:
                first = self._insert_after(self.head, 1)
            first.keys.add(key)
            self.key_bucket[key] = first

    def minus(self, key):
        if key not in self.key_bucket:
            return
        cur = self.key_bucket[key]
        if cur.count == 1:
            del self.key_bucket[key]
        else:
            prv = cur.prev
            if prv is self.head or prv.count != cur.count - 1:
                prv = self._insert_after(cur.prev, cur.count - 1)
            prv.keys.add(key)
            self.key_bucket[key] = prv
        cur.keys.discard(key)
        if not cur.keys:
            self._remove(cur)

    def get_max(self):
        return "" if self.tail.prev is self.head else min(self.tail.prev.keys)

    def get_min(self):
        return "" if self.head.next is self.tail else min(self.head.next.keys)


if __name__ == "__main__":
    a = AllOne()
    for _ in range(3):
        a.plus("a")
    a.plus("b")
    print("get_max:", a.get_max())
    print("get_min:", a.get_min())
    a.minus("a"); a.minus("a")
    print("get_max:", a.get_max())
    print("get_min:", a.get_min())
