# Day 871: Soundex phonetic algorithm: keep first letter, code rest, dedupe, pad to 3 digits.
# Time O(n), Space O(1) extra per name.

_CODE = {}
for chs, d in [("bfpv", 1), ("cgjkqsxz", 2), ("dt", 3), ("l", 4), ("mn", 5), ("r", 6)]:
    for ch in chs:
        _CODE[ch] = d


def code(c):
    return _CODE.get(c.lower(), 0)


def soundex(name):
    i = 0
    while i < len(name) and not name[i].isalpha():
        i += 1
    if i >= len(name):
        return ""
    res = [name[i].upper()]
    prev = code(name[i])
    j = i + 1
    while j < len(name) and len(res) < 4:
        c = name[j].lower()
        j += 1
        if not c.isalpha():
            continue
        if c in ("h", "w"):
            continue
        d = code(c)
        if d == 0:
            prev = 0
            continue
        if d != prev:
            res.append(str(d))
        prev = d
    while len(res) < 4:
        res.append("0")
    return "".join(res[:4])


if __name__ == "__main__":
    print(soundex("Jackson"))
    print(soundex("Jaxen"))
