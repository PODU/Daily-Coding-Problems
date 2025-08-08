# Day 82: readN(n) built on read7() by buffering leftover characters between calls.
# Time O(n) per call amortized, Space O(7) buffer.


class Reader:
    def __init__(self, file):
        self.file = file
        self.pos = 0
        self.buffer = ""

    def read7(self):
        # Returns up to 7 characters from the file, advancing the cursor.
        chunk = self.file[self.pos:self.pos + 7]
        self.pos += len(chunk)
        return chunk

    def read_n(self, n):
        result = []
        size = 0
        while size < n:
            if not self.buffer:
                chunk = self.read7()
                if not chunk:
                    break  # EOF
                self.buffer = chunk
            take = min(len(self.buffer), n - size)
            result.append(self.buffer[:take])
            self.buffer = self.buffer[take:]
            size += take
        return "".join(result)


if __name__ == "__main__":
    r = Reader("Hello world")
    print('"%s"' % r.read_n(7))  # "Hello w"
    print('"%s"' % r.read_n(7))  # "orld"
    print('"%s"' % r.read_n(7))  # ""
