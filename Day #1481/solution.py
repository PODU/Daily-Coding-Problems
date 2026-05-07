# Day 1481: Reverse words while keeping delimiters in their original positions.
# Tokenize into word-runs and delimiter chars, reverse only the word tokens,
# then re-emit in original layout. Handles trailing/consecutive delimiters.
# Time O(N), Space O(N).

def reverse_words(s, delims):
    tokens = []  # (is_word, text)
    i, n = 0, len(s)
    while i < n:
        if s[i] in delims:
            tokens.append((False, s[i]))
            i += 1
        else:
            j = i
            while j < n and s[j] not in delims:
                j += 1
            tokens.append((True, s[i:j]))
            i = j
    words = [t for is_w, t in tokens if is_w][::-1]
    out, k = [], 0
    for is_w, t in tokens:
        if is_w:
            out.append(words[k])
            k += 1
        else:
            out.append(t)
    return "".join(out)


if __name__ == "__main__":
    d = set("/:")
    print(reverse_words("hello/world:here", d))   # here/world:hello
    print(reverse_words("hello/world:here/", d))  # here/world:hello/
    print(reverse_words("hello//world:here", d))  # here//world:hello
