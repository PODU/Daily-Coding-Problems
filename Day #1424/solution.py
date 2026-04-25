# Day 1424: URL shortener (shorten -> 6-char code, restore -> original or None).
# Approach: two hash maps + base62 counter; same URL reuses its code. O(1) amortized per op.

ALPHABET = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"


class URLShortener:
    def __init__(self):
        self.counter = 0
        self.url_to_short = {}
        self.short_to_url = {}

    def _encode(self, n):
        s = ["0"] * 6
        for i in range(5, -1, -1):
            s[i] = ALPHABET[n % 62]
            n //= 62
        return "".join(s)

    def shorten(self, url):
        if url in self.url_to_short:
            return self.url_to_short[url]  # same URL -> same code
        code = self._encode(self.counter)
        self.counter += 1
        self.url_to_short[url] = code
        self.short_to_url[code] = url
        return code

    def restore(self, code):
        return self.short_to_url.get(code)


if __name__ == "__main__":
    s = URLShortener()
    a = s.shorten("https://example.com/page")
    b = s.shorten("https://example.com/page")
    print(a)                       # 000000
    print(a == b)                  # True
    print(s.restore(a))            # https://example.com/page
    print(s.restore("zzzzzz"))     # None
