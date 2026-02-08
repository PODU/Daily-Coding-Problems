# Day 1044: Reverse words but keep delimiters fixed in place: extract words, reverse the list,
# rebuild keeping delimiter chars at original positions. O(n) time, O(n) space.

DELIMS = {'/', ':'}


def reverse_words(s):
    words = []
    cur = []
    for c in s:
        if c in DELIMS:
            if cur:
                words.append("".join(cur))
                cur = []
        else:
            cur.append(c)
    if cur:
        words.append("".join(cur))
    words.reverse()

    out = []
    wi = 0
    i = 0
    n = len(s)
    while i < n:
        if s[i] in DELIMS:
            out.append(s[i])
            i += 1
        else:
            out.append(words[wi])
            wi += 1
            while i < n and s[i] not in DELIMS:
                i += 1
    return "".join(out)


def main():
    tests = ["hello/world:here", "hello/world:here/", "hello//world:here"]
    for t in tests:
        print(f"{t} -> {reverse_words(t)}")


if __name__ == "__main__":
    main()
