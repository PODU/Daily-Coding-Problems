# Day 139: Wrap an iterator and buffer one element for peek(). next/hasNext/peek all O(1).
# Time O(1) per op; Space O(1).


class Iterator(object):
    def __init__(self, data):
        self._data = data
        self._idx = 0

    def next(self):
        val = self._data[self._idx]
        self._idx += 1
        return val

    def hasNext(self):
        return self._idx < len(self._data)


class PeekableInterface(object):
    def __init__(self, iterator):
        self._it = iterator
        self._buffer = None
        self._has_buffer = False

    def peek(self):
        if not self._has_buffer:
            self._buffer = self._it.next()
            self._has_buffer = True
        return self._buffer

    def next(self):
        if self._has_buffer:
            self._has_buffer = False
            return self._buffer
        return self._it.next()

    def hasNext(self):
        return self._has_buffer or self._it.hasNext()


if __name__ == "__main__":
    p = PeekableInterface(Iterator([1, 2, 3]))
    print("peek={} next={} peek={} next={} next={} hasNext={}".format(
        p.peek(), p.next(), p.peek(), p.next(), p.next(),
        str(p.hasNext()).lower()))
    # peek=1 next=1 peek=2 next=2 next=3 hasNext=false
