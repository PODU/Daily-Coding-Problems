# Day 1569: URL shortener: base62 6-char codes from a counter, two dicts (forward + reverse) for dedupe.
# Time O(1) per shorten/restore, space O(n).
ALPHABET = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"


class URLShortener:
    def __init__(self):
        self.counter = self._decode("abcdef")
        self.code_to_url = {}
        self.url_to_code = {}

    def _encode(self, num):
        s = [""] * 6
        for i in range(5, -1, -1):
            s[i] = ALPHABET[num % 62]
            num //= 62
        return "".join(s)

    def _decode(self, s):
        num = 0
        for c in s:
            num = num * 62 + ALPHABET.index(c)
        return num

    def shorten(self, url):
        if url in self.url_to_code:
            return self.url_to_code[url]
        code = self._encode(self.counter)
        self.counter += 1
        self.code_to_url[code] = url
        self.url_to_code[url] = code
        return code

    def restore(self, code):
        return self.code_to_url.get(code)


if __name__ == "__main__":
    s = URLShortener()
    code = s.shorten("https://www.example.com/some/long/path")
    print(code)
    r1 = s.restore(code)
    print(r1 if r1 is not None else "null")
    r2 = s.restore("XXXXXX")
    print(r2 if r2 is not None else "null")
