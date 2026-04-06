# Day 1309: Soundex phonetic algorithm: keep first letter, map consonants to digits,
# collapse same-code runs, drop vowels, pad/truncate to 3 digits. O(n) time.

_CODES = {}
for letters, d in [("bfpv", 1), ("cgjkqsxz", 2), ("dt", 3),
                   ("l", 4), ("mn", 5), ("r", 6)]:
    for ch in letters:
        _CODES[ch] = d


def code(c):
    return _CODES.get(c.lower(), 0)  # vowels, y, h, w -> 0


def soundex(name):
    if not name:
        return ""
    out = [name[0].upper()]
    prev = code(name[0])
    for ch in name[1:]:
        if len(out) >= 4:
            break
        lc = ch.lower()
        c = code(lc)
        if c != 0 and c != prev:
            out.append(str(c))
        if lc in ("h", "w"):
            continue  # do not reset prev
        prev = c
    out += ["0"] * (4 - len(out))
    return "".join(out[:4])


if __name__ == "__main__":
    print(soundex("Jackson"))  # J250
    print(soundex("Jaxen"))    # J250
