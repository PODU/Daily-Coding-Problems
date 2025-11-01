# Day 531: readN(n) on top of read7(): buffer leftover chars between calls.
# Time O(n) per readN call, Space O(1) extra (small buffer).


class Reader:
    def __init__(self, content: str):
        self.content = content
        self.pos = 0       # read7 pointer
        self.buf = ""      # leftover unconsumed chars

    def read7(self) -> str:
        chunk = self.content[self.pos:self.pos + 7]
        self.pos += len(chunk)
        return chunk

    def read_n(self, n: int) -> str:
        while len(self.buf) < n:
            chunk = self.read7()
            if not chunk:
                break
            self.buf += chunk
        out, self.buf = self.buf[:n], self.buf[n:]
        return out


if __name__ == "__main__":
    r = Reader("Hello world")
    print('"' + r.read_n(7) + '"')
    print('"' + r.read_n(7) + '"')
    print('"' + r.read_n(7) + '"')
