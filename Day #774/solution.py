# Day 774: Implement readN(n) on top of a read7() primitive.
# Buffer leftover chars from read7 between calls. O(n) per readN call.


class Reader:
    def __init__(self, file):
        self.file = file
        self.pos = 0
        self.buf = ""

    def read7(self):
        r = self.file[self.pos:self.pos + 7]
        self.pos += len(r)
        return r

    def read_n(self, n):
        out = []
        size = 0
        while size < n:
            if not self.buf:
                self.buf = self.read7()
                if not self.buf:
                    break
            take = min(len(self.buf), n - size)
            out.append(self.buf[:take])
            self.buf = self.buf[take:]
            size += take
        return "".join(out)


if __name__ == "__main__":
    r = Reader("Hello world")
    print(f'"{r.read_n(7)}", "{r.read_n(7)}", "{r.read_n(7)}"')
    # "Hello w", "orld", ""
