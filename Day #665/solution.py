# Day 665: URL shortener. Base62-encode an incrementing counter into a 6-char code;
# dedup with url->code map so the same URL maps once. shorten/restore O(1) avg.
A = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"


class Shortener:
    def __init__(self):
        self.code2url = {}
        self.url2code = {}
        self.counter = 916132831  # seeds a 6-char base62 code

    def _encode(self, n):
        s = []
        for _ in range(6):
            s.append(A[n % 62])
            n //= 62
        return "".join(reversed(s))

    def shorten(self, url):
        if url in self.url2code:
            return self.url2code[url]
        code = self._encode(self.counter)
        self.counter += 1
        self.code2url[code] = url
        self.url2code[url] = code
        return code

    def restore(self, code):
        return self.code2url.get(code)  # None if absent


if __name__ == "__main__":
    s = Shortener()
    c = s.shorten("https://example.com/long/path")
    print("short:", c)
    print("restore:", s.restore(c))
    print("restore(zzzzzz):", s.restore("zzzzzz"))  # None
