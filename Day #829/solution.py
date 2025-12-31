# Day 829: O(1) data structure with plus/minus/get_max/get_min.
# Doubly-linked list of count-buckets (each holds a set of keys) + key->bucket map.
# All operations O(1) time; O(K) space for K distinct keys.


class Bucket:
    def __init__(self, count):
        self.count = count
        self.keys = set()
        self.prev = None
        self.next = None


class AllOne:
    def __init__(self):
        # Sentinel head/tail; head.next = lowest count, tail.prev = highest count.
        self.head = Bucket(float("-inf"))
        self.tail = Bucket(float("inf"))
        self.head.next = self.tail
        self.tail.prev = self.head
        self.key_bucket = {}  # key -> Bucket

    def _insert_after(self, node, count):
        b = Bucket(count)
        b.prev = node
        b.next = node.next
        node.next.prev = b
        node.next = b
        return b

    def _remove(self, node):
        node.prev.next = node.next
        node.next.prev = node.prev

    def plus(self, key):
        if key in self.key_bucket:
            cur = self.key_bucket[key]
            new_count = cur.count + 1
            nxt = cur.next
            if nxt.count != new_count:
                nxt = self._insert_after(cur, new_count)
            nxt.keys.add(key)
            self.key_bucket[key] = nxt
            cur.keys.discard(key)
            if not cur.keys:
                self._remove(cur)
        else:
            first = self.head.next
            if first.count != 1:
                first = self._insert_after(self.head, 1)
            first.keys.add(key)
            self.key_bucket[key] = first

    def minus(self, key):
        if key not in self.key_bucket:
            return
        cur = self.key_bucket[key]
        if cur.count == 1:
            cur.keys.discard(key)
            del self.key_bucket[key]
            if not cur.keys:
                self._remove(cur)
            return
        new_count = cur.count - 1
        prev = cur.prev
        if prev.count != new_count:
            prev = self._insert_after(cur.prev, new_count)
        prev.keys.add(key)
        self.key_bucket[key] = prev
        cur.keys.discard(key)
        if not cur.keys:
            self._remove(cur)

    def get_max(self):
        if self.tail.prev is self.head:
            return ""
        return min(self.tail.prev.keys)

    def get_min(self):
        if self.head.next is self.tail:
            return ""
        return min(self.head.next.keys)


def main():
    ao = AllOne()
    ao.plus("a")
    ao.plus("b")
    ao.plus("b")
    print("get_max:", ao.get_max())  # b (count 2)
    print("get_min:", ao.get_min())  # a (count 1)
    ao.minus("b")
    ao.minus("b")
    print("get_max:", ao.get_max())  # a (b removed)


if __name__ == "__main__":
    main()
