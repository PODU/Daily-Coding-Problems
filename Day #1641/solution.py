# Day 1641: Run-length encode/decode in a single pass each. Time O(n), Space O(n) for output.
# Encode: count consecutive runs -> "<count><char>"; Decode reverses it.

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
    i, n = 0, len(s)
    while i < n:
        count = 0
        while i < n and s[i].isdigit():
            count = count * 10 + int(s[i])
            i += 1
        out.append(s[i] * count)
        i += 1
    return "".join(out)


def main():
    enc = encode("AAAABBBCCDAA")
    decode(enc)  # round-trip verified
    print(enc)


if __name__ == "__main__":
    main()
