# Day 1245: Full text justification: greedily pack words per line, distribute spaces
# evenly with extras to the left. Time O(total chars), Space O(output).


def justify(words, k):
    lines = []
    i, n = 0, len(words)
    while i < n:
        j = i
        length = 0
        # pack as many words as fit (min one space between)
        while j < n and length + len(words[j]) + (j - i) <= k:
            length += len(words[j])
            j += 1
        gaps = j - i - 1
        if gaps == 0:
            lines.append(words[i] + ' ' * (k - len(words[i])))
        else:
            spaces = k - length
            base, extra = divmod(spaces, gaps)
            parts = []
            for w in range(i, j - 1):
                parts.append(words[w])
                parts.append(' ' * (base + (1 if (w - i) < extra else 0)))
            parts.append(words[j - 1])
            lines.append(''.join(parts))
        i = j
    return lines


if __name__ == "__main__":
    words = ["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"]
    for line in justify(words, 16):
        print(f'"{line}"')
