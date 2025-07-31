# Day 55: URL shortener. 6-char base62 code; same URL maps to same code.
# Time: O(1) amortized per op, Space: O(n).
import random
import string

ALPHA = string.ascii_letters + string.digits


class URLShortener:
    def __init__(self):
        self.to_long = {}
        self.to_short = {}

    def _rand_code(self):
        return "".join(random.choice(ALPHA) for _ in range(6))

    def shorten(self, url):
        if url in self.to_short:
            return self.to_short[url]  # same URL -> same code
        code = self._rand_code()
        while code in self.to_long:
            code = self._rand_code()
        self.to_long[code] = url
        self.to_short[url] = code
        return code

    def restore(self, code):
        return self.to_long.get(code)  # None if unknown


if __name__ == "__main__":
    s = URLShortener()
    a = s.shorten("https://example.com/foo")
    b = s.shorten("https://example.com/foo")  # same URL twice
    print("same code reused:", a == b)
    print("restore:", s.restore(a))
    print("restore unknown:", s.restore("zzzzzz"))
