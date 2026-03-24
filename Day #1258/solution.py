# Day 1258: Peeking iterator: buffer one element ahead. peek/next/hasNext all O(1) time, O(1) extra space.

class PeekableInterface:
    def __init__(self, iterator):
        self._it = iter(iterator)
        self._buffer = None
        self._has_buffer = False

    def peek(self):
        if not self._has_buffer:
            self._buffer = next(self._it)
            self._has_buffer = True
        return self._buffer

    def next(self):
        if self._has_buffer:
            self._has_buffer = False
            return self._buffer
        return next(self._it)

    def hasNext(self):
        if self._has_buffer:
            return True
        try:
            self._buffer = next(self._it)
            self._has_buffer = True
            return True
        except StopIteration:
            return False


def main():
    p = PeekableInterface([1, 2, 3])
    print(p.peek())
    print(p.next())
    print(p.next())
    print(p.peek())
    print(str(p.hasNext()).lower())
    print(p.next())
    print(str(p.hasNext()).lower())


if __name__ == "__main__":
    main()
