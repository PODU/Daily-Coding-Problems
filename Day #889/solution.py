# Day 889: Run-length encoding/decoding. Single pass over the string.
# Time: O(n) encode/decode, Space: O(n) for output.
import re


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
    for cnt, ch in re.findall(r"(\d+)([A-Za-z])", s):
        out.append(ch * int(cnt))
    return "".join(out)


if __name__ == "__main__":
    inp = "AAAABBBCCDAA"
    enc = encode(inp)
    print(enc)
    print(decode(enc))
