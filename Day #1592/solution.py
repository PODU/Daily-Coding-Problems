# Day 1592: readN using read7: buffer leftover chars from read7 between calls; pull until n satisfied or EOF.
# Time O(n) per readN call.


class Reader:
    def __init__(self, file: str):
        self.file = file
        self.pos = 0       # internal pointer for read7
        self.buf = ""      # leftover chars buffered between readN calls

    def read7(self) -> str:
        # returns up to 7 chars, advances pointer, "" at EOF
        res = self.file[self.pos:self.pos + 7]
        self.pos += len(res)
        return res

    def readN(self, n: int) -> str:
        # read exactly n chars (or fewer at EOF), buffering leftovers
        out = []
        size = 0
        while size < n:
            if not self.buf:
                self.buf = self.read7()
                if not self.buf:
                    break  # EOF
            take = min(n - size, len(self.buf))
            out.append(self.buf[:take])
            self.buf = self.buf[take:]
            size += take
        return "".join(out)


if __name__ == "__main__":
    r1 = Reader("Hello world")
    print('read7: "%s"' % r1.read7())
    print('read7: "%s"' % r1.read7())
    print('read7: "%s"' % r1.read7())

    r2 = Reader("Hello world")
    print('readN(5): "%s"' % r2.readN(5))
    print('readN(100): "%s"' % r2.readN(100))
    print('readN(3): "%s"' % r2.readN(3))
