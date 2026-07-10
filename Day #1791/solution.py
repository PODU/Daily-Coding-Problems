# Day 1791: All O(1) data structure (plus / minus / get_max / get_min).
#
# The trick is to keep a doubly linked list of "buckets", one per distinct
# count, sorted by count. Each bucket holds the set of keys that currently
# have that count. A dict maps each key to the bucket it lives in.
#
# Incrementing a key just moves it to the neighbouring bucket (creating it
# if the neighbour has the wrong count), so every operation only touches a
# constant number of buckets. Min and max are simply the buckets sitting
# next to the sentinels at each end of the list.


class Bucket:
    """A node in the linked list: one count, and every key that has it."""

    __slots__ = ("count", "keys", "prev", "next")

    def __init__(self, count):
        self.count = count
        self.keys = set()
        self.prev = None
        self.next = None


class AllOne:
    def __init__(self):
        # Sentinel buckets at both ends so we never have to special-case
        # inserting at the front or back of the list.
        self.head = Bucket(float("-inf"))
        self.tail = Bucket(float("inf"))
        self.head.next = self.tail
        self.tail.prev = self.head
        self.key_bucket = {}  # key -> the bucket it currently lives in

    def _insert_after(self, node, count):
        """Splice a fresh bucket for `count` right after `node`."""
        b = Bucket(count)
        b.prev = node
        b.next = node.next
        node.next.prev = b
        node.next = b
        return b

    def _remove(self, b):
        """Unlink an empty bucket from the list."""
        b.prev.next = b.next
        b.next.prev = b.prev

    def plus(self, key):
        if key in self.key_bucket:
            # Key exists: move it one bucket to the right (count + 1).
            cur = self.key_bucket[key]
            nxt = cur.next
            if nxt is self.tail or nxt.count != cur.count + 1:
                # No bucket for count+1 yet, so make one next door.
                nxt = self._insert_after(cur, cur.count + 1)
            nxt.keys.add(key)
            self.key_bucket[key] = nxt
            cur.keys.discard(key)
            if not cur.keys:
                # The bucket we just left is empty now, drop it.
                self._remove(cur)
        else:
            # Brand new key: it belongs in the count-1 bucket at the front.
            first = self.head.next
            if first is self.tail or first.count != 1:
                first = self._insert_after(self.head, 1)
            first.keys.add(key)
            self.key_bucket[key] = first

    def minus(self, key):
        if key not in self.key_bucket:
            return  # decrementing something we don't have is a no-op

        cur = self.key_bucket[key]
        cur.keys.discard(key)

        if cur.count == 1:
            # Count would drop to zero, so the key disappears entirely.
            del self.key_bucket[key]
        else:
            # Move the key one bucket to the left (count - 1).
            prv = cur.prev
            if prv is self.head or prv.count != cur.count - 1:
                prv = self._insert_after(cur.prev, cur.count - 1)
            prv.keys.add(key)
            self.key_bucket[key] = prv

        if not cur.keys:
            self._remove(cur)

    def get_max(self):
        # The bucket just before the tail sentinel holds the highest count.
        if self.tail.prev is self.head:
            return ""
        # Any key in the bucket would do; min() keeps the output deterministic.
        return min(self.tail.prev.keys)

    def get_min(self):
        if self.head.next is self.tail:
            return ""
        return min(self.head.next.keys)


if __name__ == "__main__":
    a = AllOne()
    a.plus("apple")
    a.plus("apple")
    a.plus("banana")
    print(f"max={a.get_max()} min={a.get_min()}")  # apple / banana

    a.plus("banana")
    a.plus("banana")
    print(f"max={a.get_max()} min={a.get_min()}")  # banana / apple

    a.minus("apple")
    a.minus("apple")
    print(f"max={a.get_max()} min={a.get_min()}")  # banana / banana
