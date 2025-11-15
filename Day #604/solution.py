# Day 604: Soundex phonetic encoding (letter + 3 digits).
# Approach: keep first letter, code consonants, drop repeats/vowels, pad. Time O(L), Space O(L).

_CODES = {}
for letters, d in [("BFPV", '1'), ("CGJKQSXZ", '2'), ("DT", '3'),
                   ("L", '4'), ("MN", '5'), ("R", '6')]:
    for ch in letters:
        _CODES[ch] = d


def code(c):
    if c in _CODES:
        return _CODES[c]
    if c in "HW":
        return 'S'   # transparent
    return '0'       # vowels A E I O U Y


def soundex(name):
    up = [c.upper() for c in name if c.isalpha()]
    if not up:
        return "0000"
    res = [up[0]]
    prev = code(up[0])
    for ch in up[1:]:
        if len(res) >= 4:
            break
        c = code(ch)
        if c in '123456':
            if c != prev:
                res.append(c)
            prev = c
        elif c == '0':
            prev = '0'
        # 'S' (h, w): transparent
    while len(res) < 4:
        res.append('0')
    return ''.join(res[:4])


def main():
    print(soundex("Jackson"))  # J250
    print(soundex("Jaxen"))    # J250


if __name__ == '__main__':
    main()
