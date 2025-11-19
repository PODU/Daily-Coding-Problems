# Day 627: Peekable iterator: cache one element ahead so peek() returns next without advancing.
# next/peek/hasNext all O(1).

_SENTINEL = object()


class Peekable:
    def __init__(self, iterable):
        self._it = iter(iterable)
        self._cache = _SENTINEL

    def has_next(self):
        if self._cache is _SENTINEL:
            self._cache = next(self._it, _SENTINEL)
        return self._cache is not _SENTINEL

    def peek(self):
        if self._cache is _SENTINEL:
            self._cache = next(self._it)
        return self._cache

    def next(self):
        if self._cache is not _SENTINEL:
            v, self._cache = self._cache, _SENTINEL
            return v
        return next(self._it)


if __name__ == "__main__":
    it = Peekable([1, 2, 3, 4])
    print(it.peek())      # 1
    print(it.next())      # 1
    print(it.next())      # 2
    print(it.peek())      # 3
    print(it.next())      # 3
    print(it.has_next())  # True
    print(it.next())      # 4
    print(it.has_next())  # False
