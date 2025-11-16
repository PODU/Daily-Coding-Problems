# Day 611: All O(1) structure (plus / minus / get_max / get_min).
# Approach: doubly-linked list of value-buckets (set of keys) + key->bucket map. All ops O(1).


class Bucket:
    __slots__ = ('value', 'keys', 'prev', 'next')

    def __init__(self, value):
        self.value = value
        self.keys = set()
        self.prev = None
        self.next = None


class AllOne:
    def __init__(self):
        self.head = Bucket(0)
        self.tail = Bucket(0)
        self.head.next = self.tail
        self.tail.prev = self.head
        self.key_to_bucket = {}

    def _insert_after(self, node, value):
        b = Bucket(value)
        b.prev, b.next = node, node.next
        node.next.prev = b
        node.next = b
        return b

    def _remove(self, node):
        node.prev.next = node.next
        node.next.prev = node.prev

    def plus(self, key):
        if key in self.key_to_bucket:
            cur = self.key_to_bucket[key]
            nxt = cur.next
            if nxt is self.tail or nxt.value != cur.value + 1:
                nxt = self._insert_after(cur, cur.value + 1)
            nxt.keys.add(key)
            self.key_to_bucket[key] = nxt
            cur.keys.discard(key)
            if not cur.keys:
                self._remove(cur)
        else:
            first = self.head.next
            if first is self.tail or first.value != 1:
                first = self._insert_after(self.head, 1)
            first.keys.add(key)
            self.key_to_bucket[key] = first

    def minus(self, key):
        if key not in self.key_to_bucket:
            return
        cur = self.key_to_bucket[key]
        if cur.value == 1:
            del self.key_to_bucket[key]
        else:
            prv = cur.prev
            if prv is self.head or prv.value != cur.value - 1:
                prv = self._insert_after(cur.prev, cur.value - 1)
            prv.keys.add(key)
            self.key_to_bucket[key] = prv
        cur.keys.discard(key)
        if not cur.keys:
            self._remove(cur)

    def get_max(self):
        return "" if self.tail.prev is self.head else min(self.tail.prev.keys)

    def get_min(self):
        return "" if self.head.next is self.tail else min(self.head.next.keys)


def main():
    a = AllOne()
    a.plus("a"); a.plus("b"); a.plus("a")  # a=2, b=1
    print(a.get_max())  # a
    print(a.get_min())  # b


if __name__ == '__main__':
    main()
