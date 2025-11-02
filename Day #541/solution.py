# Day 541: Run-length encoding/decoding. Scan runs to build count+char; parse digits to expand.
# Time O(n) encode, O(output) decode. Space O(n).


def encode(s):
    out = []
    i, n = 0, len(s)
    while i < n:
        j = i
        while j < n and s[j] == s[i]:
            j += 1
        out.append(str(j - i) + s[i])
        i = j
    return "".join(out)


def decode(s):
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
    original = "AAAABBBCCDAA"
    enc = encode(original)
    print(enc)                                # 4A3B2C1D2A
    print(str(decode(enc) == original).lower())
