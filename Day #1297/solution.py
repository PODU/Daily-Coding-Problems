# Day 1297: Implement readN(n) on top of a read7() primitive.
# Keep a leftover buffer of unused chars; refill via read7 until n chars (or EOF). O(n) amortized.


class Reader:
    def __init__(self, content: str):
        self.file = content
        self.pos = 0

    def read7(self) -> str:  # up to 7 chars, "" at EOF
        r = self.file[self.pos:self.pos + 7]
        self.pos += len(r)
        return r


class NReader:
    def __init__(self, reader: Reader):
        self.r = reader
        self.buf = ""

    def read_n(self, n: int) -> str:
        while len(self.buf) < n:
            chunk = self.r.read7()
            if not chunk:
                break
            self.buf += chunk
        out, self.buf = self.buf[:n], self.buf[n:]
        return out


if __name__ == "__main__":
    nr = NReader(Reader("Hello world"))
    print(f"'{nr.read_n(5)}'")   # 'Hello'
    print(f"'{nr.read_n(4)}'")   # ' wor'
    print(f"'{nr.read_n(10)}'")  # 'ld'
