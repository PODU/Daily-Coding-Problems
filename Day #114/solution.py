# Day 114: Reverse words but keep delimiters fixed. Collect words, reverse list,
# re-emit walking original structure. O(n) time, O(n) space.
def reverse_keep_delims(s, delim):
    words = []
    cur = []
    for c in s:
        if c in delim:
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
    i, n = 0, len(s)
    while i < n:
        if s[i] in delim:
            out.append(s[i])
            i += 1
        else:
            out.append(words[wi])
            wi += 1
            while i < n and s[i] not in delim:
                i += 1
    return "".join(out)


if __name__ == "__main__":
    d = {"/", ":"}
    print(reverse_keep_delims("hello/world:here", d))   # here/world:hello
    print(reverse_keep_delims("hello/world:here/", d))  # here/world:hello/
    print(reverse_keep_delims("hello//world:here", d))  # here//world:hello
