# Day 623: Full text justification: greedily pack words per line, distribute spaces evenly
# with extra spaces favoring left gaps; last/single word left-justified. Time O(total chars).


def justify(words, k):
    res = []
    n = len(words)
    i = 0
    while i < n:
        j = i
        line_len = len(words[i])
        while j + 1 < n and line_len + 1 + len(words[j + 1]) <= k:
            j += 1
            line_len += 1 + len(words[j])
        gaps = j - i
        if gaps == 0:
            res.append(words[i].ljust(k))
        else:
            total_spaces = k - sum(len(words[w]) for w in range(i, j + 1))
            base, extra = divmod(total_spaces, gaps)
            line = []
            for w in range(i, j + 1):
                line.append(words[w])
                if w < j:
                    sp = base + (1 if w - i < extra else 0)
                    line.append(' ' * sp)
            res.append(''.join(line))
        i = j + 1
    return res


if __name__ == "__main__":
    words = ["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"]
    for line in justify(words, 16):
        print('"' + line + '"')
