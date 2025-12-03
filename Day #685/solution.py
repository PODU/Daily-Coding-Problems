# Day 685: Reverse words between delimiters while keeping delimiters fixed in position.
# Tokenize (words = maximal non-delim runs incl. interior empties), reverse word list, reassemble. O(n) time/space.

def reverse_words(s, delims):
    tokens = []  # (is_delim, value)
    buf = []
    for c in s:
        if c in delims:
            tokens.append((False, ''.join(buf)))
            tokens.append((True, c))
            buf = []
        else:
            buf.append(c)
    if buf:
        tokens.append((False, ''.join(buf)))

    words = [v for is_d, v in tokens if not is_d]
    words.reverse()

    out, wi = [], 0
    for is_d, v in tokens:
        if is_d:
            out.append(v)
        else:
            out.append(words[wi]); wi += 1
    return ''.join(out)


if __name__ == '__main__':
    d = {'/', ':'}
    for t in ["hello/world:here", "hello/world:here/", "hello//world:here"]:
        print(f"{t} -> {reverse_words(t, d)}")
