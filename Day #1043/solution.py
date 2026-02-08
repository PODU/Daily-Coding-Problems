# Day 1043: PeekableIterator: wrap an iterator and cache one element ahead so peek() returns
# the next value without consuming it. O(1) per op, O(1) extra space.

class PeekableInterface:
    _SENTINEL = object()

    def __init__(self, iterable):
        self._it = iter(iterable)
        self._cached = self._SENTINEL

    def has_next(self):
        if self._cached is not self._SENTINEL:
            return True
        try:
            self._cached = next(self._it)
            return True
        except StopIteration:
            return False

    def next(self):
        if self._cached is not self._SENTINEL:
            v = self._cached
            self._cached = self._SENTINEL
            return v
        return next(self._it)

    def peek(self):
        if self._cached is self._SENTINEL:
            self._cached = next(self._it)
        return self._cached


def main():
    it = PeekableInterface([1, 2, 3])
    print(f"peek() -> {it.peek()}")
    print(f"next() -> {it.next()}")
    print(f"peek() -> {it.peek()}")
    print(f"next() -> {it.next()}")
    print(f"next() -> {it.next()}")
    print(f"hasNext() -> {str(it.has_next()).lower()}")


if __name__ == "__main__":
    main()
