# Day 367: Lazily merge two sorted iterators into one sorted iterator.
# A generator peeks the head of each iterator and yields the smaller; nothing is
# buffered into memory. Time O(n+m), Space O(1).
_SENTINEL = object()


def merge_iterators(a, b):
    a, b = iter(a), iter(b)
    x = next(a, _SENTINEL)
    y = next(b, _SENTINEL)
    while x is not _SENTINEL or y is not _SENTINEL:
        if y is _SENTINEL or (x is not _SENTINEL and x <= y):
            yield x
            x = next(a, _SENTINEL)
        else:
            yield y
            y = next(b, _SENTINEL)


if __name__ == "__main__":
    foo = iter([5, 10, 15])
    bar = iter([3, 8, 9])
    for num in merge_iterators(foo, bar):
        print(num)  # 3 5 8 9 10 15
