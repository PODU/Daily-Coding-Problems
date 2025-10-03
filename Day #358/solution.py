# Day 358: All O(1) structure: doubly-linked list of count-buckets (increasing), each holds an ordered key set; dict key->bucket.
# plus/minus move key to adjacent bucket. get_max=last bucket, get_min=first bucket. All O(1).

class Bucket:
    __slots__ = ("count", "keys", "prev", "next")
    def __init__(self, count):
        self.count = count
        self.keys = dict()  # ordered set (insertion order)
        self.prev = None
        self.next = None

class AllOne:
    def __init__(self):
        self.head = Bucket(float("-inf"))  # smallest
        self.tail = Bucket(float("inf"))   # largest
        self.head.next = self.tail
        self.tail.prev = self.head
        self.key_bucket = {}

    def _insert_after(self, b, count):
        nb = Bucket(count)
        nb.prev, nb.next = b, b.next
        b.next.prev = nb
        b.next = nb
        return nb

    def _remove(self, b):
        b.prev.next = b.next
        b.next.prev = b.prev

    def plus(self, key):
        if key not in self.key_bucket:
            first = self.head.next
            if first is self.tail or first.count != 1:
                first = self._insert_after(self.head, 1)
            first.keys[key] = True
            self.key_bucket[key] = first
            return
        cur = self.key_bucket[key]
        nxt = cur.next
        if nxt is self.tail or nxt.count != cur.count + 1:
            nxt = self._insert_after(cur, cur.count + 1)
        nxt.keys[key] = True
        self.key_bucket[key] = nxt
        del cur.keys[key]
        if not cur.keys:
            self._remove(cur)

    def minus(self, key):
        if key not in self.key_bucket:
            return
        cur = self.key_bucket[key]
        if cur.count == 1:
            del cur.keys[key]
            del self.key_bucket[key]
            if not cur.keys:
                self._remove(cur)
            return
        prv = cur.prev
        if prv is self.head or prv.count != cur.count - 1:
            prv = self._insert_after(cur.prev, cur.count - 1)
        prv.keys[key] = True
        self.key_bucket[key] = prv
        del cur.keys[key]
        if not cur.keys:
            self._remove(cur)

    def get_max(self):
        return "" if self.tail.prev is self.head else min(self.tail.prev.keys)

    def get_min(self):
        return "" if self.head.next is self.tail else min(self.head.next.keys)

if __name__ == "__main__":
    a = AllOne()
    a.plus("a"); a.plus("b"); a.plus("b")
    a.plus("c"); a.plus("c"); a.plus("c")
    print("max=" + a.get_max())
    print("min=" + a.get_min())
    a.minus("c"); a.minus("c")
    print("max=" + a.get_max())
