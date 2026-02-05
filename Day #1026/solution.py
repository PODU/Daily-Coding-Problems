# Day 1026: Full text justification.
# Greedy line packing; distribute extra spaces evenly, leftover from the left.
# Time O(total chars), Space O(total chars).


def justify(words, k):
    res = []
    i, n = 0, len(words)
    while i < n:
        j = i
        line_len = len(words[i])
        while j + 1 < n and line_len + 1 + len(words[j + 1]) <= k:
            line_len += 1 + len(words[j + 1])
            j += 1
        cnt = j - i + 1
        if cnt == 1:
            res.append(words[i] + " " * (k - len(words[i])))
        else:
            total_chars = sum(len(words[w]) for w in range(i, j + 1))
            spaces = k - total_chars
            gaps = cnt - 1
            base, extra = divmod(spaces, gaps)
            parts = []
            for idx, w in enumerate(range(i, j + 1)):
                parts.append(words[w])
                if w < j:
                    parts.append(" " * (base + (1 if idx < extra else 0)))
            res.append("".join(parts))
        i = j + 1
    return res


if __name__ == "__main__":
    words = ["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"]
    for line in justify(words, 16):
        print(line)
