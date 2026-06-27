# Day 1721: Reverse words while keeping delimiters in place: split into word/delimiter tokens,
# reverse only the word list, re-emit in original token order. Time O(n), Space O(n).


def reverse_words(s, delims):
    tokens = []  # list of (text, is_word)
    cur = []
    for c in s:
        if c in delims:
            if cur:
                tokens.append(("".join(cur), True))
                cur = []
            tokens.append((c, False))
        else:
            cur.append(c)
    if cur:
        tokens.append(("".join(cur), True))

    words = [t for t, w in tokens if w]
    words.reverse()

    res = []
    wi = 0
    for t, w in tokens:
        if w:
            res.append(words[wi])
            wi += 1
        else:
            res.append(t)
    return "".join(res)


if __name__ == "__main__":
    delims = {"/", ":"}
    print(reverse_words("hello/world:here", delims))
    print(reverse_words("hello/world:here/", delims))
    print(reverse_words("hello//world:here", delims))
