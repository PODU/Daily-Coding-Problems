# Day 349: Soundex: keep first letter, map consonants to digits, collapse same adjacent codes
# (h,w transparent; vowels reset), pad/truncate to 3 digits. Time O(L), Space O(1).
def code(c):
    c = c.lower()
    return {
        "b": 1, "f": 1, "p": 1, "v": 1,
        "c": 2, "g": 2, "j": 2, "k": 2, "q": 2, "s": 2, "x": 2, "z": 2,
        "d": 3, "t": 3,
        "l": 4,
        "m": 5, "n": 5,
        "r": 6,
    }.get(c, 0)  # vowels, y, w, h -> 0


def soundex(name):
    res = name[0].upper()
    prev = code(name[0])
    for c in name[1:]:
        if len(res) >= 4:
            break
        c = c.lower()
        d = code(c)
        if d != 0 and d != prev:
            res += str(d)
        if c in ("h", "w"):
            continue  # transparent: keep prev
        prev = d       # vowels reset prev to 0
    return (res + "000")[:4]


if __name__ == "__main__":
    print(soundex("Jackson"))
