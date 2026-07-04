# Day 1764: Soundex phonetic encoding (NARA rules): keep first letter, map rest to digits,
# collapse adjacent same-codes (h/w transparent), drop vowels, pad/truncate to 3 digits.
# Time: O(n) per name, Space: O(n).

def code(c):
    c = c.lower()
    if c in "bfpv":
        return 1
    if c in "cgjkqsxz":
        return 2
    if c in "dt":
        return 3
    if c == "l":
        return 4
    if c in "mn":
        return 5
    if c == "r":
        return 6
    return 0  # vowels a,e,i,o,u,y and h,w


def soundex(name):
    s = [c for c in name if c.isalpha()]
    if not s:
        return ""
    res = s[0].upper()
    prev = code(s[0])
    for ch in s[1:]:
        if len(res) >= 4:
            break
        d = code(ch)
        if d != 0 and d != prev:
            res += str(d)
        if ch.lower() not in ("h", "w"):
            prev = d
    res = (res + "000")[:4]
    return res


if __name__ == "__main__":
    print(soundex("Jackson"))
    print(soundex("Jaxen"))
