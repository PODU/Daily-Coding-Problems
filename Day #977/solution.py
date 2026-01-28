# Day 977: URL shortener: base62-encode an incrementing counter (zero-padded to 6 chars).
# Dedup via url->code map so identical URLs map to the same code.
# shorten/restore: O(L) per call (L = code length); Space: O(N) for N stored URLs.

ALPHA = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"


class URLShortener:
    def __init__(self):
        self.counter = 0
        self.url_to_code = {}
        self.code_to_url = {}

    @staticmethod
    def _encode(n):
        s = [ALPHA[0]] * 6
        i = 5
        while n > 0 and i >= 0:
            s[i] = ALPHA[n % 62]
            n //= 62
            i -= 1
        return "".join(s)

    def shorten(self, url):
        if url in self.url_to_code:        # same URL -> same code
            return self.url_to_code[url]
        code = self._encode(self.counter)
        self.counter += 1
        self.url_to_code[url] = code
        self.code_to_url[code] = url
        return code

    def restore(self, short):
        return self.code_to_url.get(short)  # None if unknown


if __name__ == "__main__":
    s = URLShortener()
    url = "https://www.example.com/some/long/path"
    code = s.shorten(url)
    print(f"shorten({url}) = {code}")
    print(f"restore({code}) = {s.restore(code)}")
    print(f"restore(zzzzzz) = {s.restore('zzzzzz') or 'null'}")
    code2 = s.shorten(url)
    print(f"shorten same url again = {code2} (same as before: {str(code == code2).lower()})")
