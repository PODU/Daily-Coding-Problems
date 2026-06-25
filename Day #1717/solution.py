# Day 1717: Fully justify text into lines of length k.
# Greedy line packing + even space distribution (extras from left).
# Time: O(total characters), Space: O(output).


def justify(words, k):
    lines = []
    n = len(words)
    i = 0
    while i < n:
        j = i
        line_len = len(words[i])
        while j + 1 < n and line_len + 1 + len(words[j + 1]) <= k:
            j += 1
            line_len += 1 + len(words[j])
        cnt = j - i + 1
        word_chars = sum(len(words[t]) for t in range(i, j + 1))
        if cnt == 1:
            line = words[i] + " " * (k - len(words[i]))
        else:
            gaps = cnt - 1
            total_spaces = k - word_chars
            base, extra = divmod(total_spaces, gaps)
            parts = []
            for t in range(i, j + 1):
                parts.append(words[t])
                if t < j:
                    sp = base + (1 if t - i < extra else 0)
                    parts.append(" " * sp)
            line = "".join(parts)
        lines.append(line)
        i = j + 1
    return lines


if __name__ == "__main__":
    words = ["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"]
    for line in justify(words, 16):
        print('"' + line + '"')
