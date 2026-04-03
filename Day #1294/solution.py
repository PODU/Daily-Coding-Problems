# Day 1294: Run-length encoding and decoding for alphabetic strings.
# Single linear scan for each direction. O(n) time, O(n) space.


def encode(s: str) -> str:
    out = []
    i, n = 0, len(s)
    while i < n:
        j = i
        while j < n and s[j] == s[i]:
            j += 1
        out.append(f"{j - i}{s[i]}")
        i = j
    return "".join(out)


def decode(s: str) -> str:
    out = []
    count = 0
    for c in s:
        if c.isdigit():
            count = count * 10 + int(c)
        else:
            out.append(c * count)
            count = 0
    return "".join(out)


if __name__ == "__main__":
    e = encode("AAAABBBCCDAA")
    print(e)            # 4A3B2C1D2A
    print(decode(e))    # AAAABBBCCDAA
