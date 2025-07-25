# Day 29: Run-length encoding/decoding in a single pass.
# Time: O(n), Space: O(n) for output.
def encode(s: str) -> str:
    res = []
    n = len(s)
    i = 0
    while i < n:
        j = i
        while j < n and s[j] == s[i]:
            j += 1
        res.append(f"{j - i}{s[i]}")
        i = j
    return "".join(res)


def decode(s: str) -> str:
    res = []
    n = len(s)
    i = 0
    while i < n:
        count = 0
        while i < n and s[i].isdigit():
            count = count * 10 + int(s[i])
            i += 1
        res.append(s[i] * count)
        i += 1
    return "".join(res)


if __name__ == "__main__":
    inp = "AAAABBBCCDAA"
    enc = encode(inp)
    print(enc)
    print(decode(enc))
