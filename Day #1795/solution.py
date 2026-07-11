# Day 1795: Peekable iterator wrapper: cache one element ahead. peek()/next()/hasNext() O(1) time, O(1) space.
class PeekableInterface:
    def __init__(self, iterator):
        self._it = iter(iterator)
        self._cached = None
        self._has_cached = False

    def hasNext(self):
        if self._has_cached:
            return True
        try:
            self._cached = next(self._it)
            self._has_cached = True
            return True
        except StopIteration:
            return False

    def peek(self):
        if not self._has_cached:
            self._cached = next(self._it)
            self._has_cached = True
        return self._cached

    def next(self):
        val = self.peek()
        self._has_cached = False
        self._cached = None
        return val


if __name__ == "__main__":
    p = PeekableInterface([1, 2, 3])
    print("peek()    ->", p.peek())
    print("next()    ->", p.next())
    print("peek()    ->", p.peek())
    print("hasNext() ->", p.hasNext())
    print("next()    ->", p.next())
    print("next()    ->", p.next())
    print("hasNext() ->", p.hasNext())
