# Day 28: Text justification: greedy line packing, distribute spaces with extras to LEFT gaps.
# Time: O(total chars), Space: O(total chars) for output.
from typing import List


def justify(words: List[str], k: int) -> List[str]:
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
            line = words[i] + " " * (k - len(words[i]))
        else:
            total_chars = sum(len(words[w]) for w in range(i, j + 1))
            total_spaces = k - total_chars
            base, extra = divmod(total_spaces, gaps)
            parts = []
            for w in range(i, j + 1):
                parts.append(words[w])
                if w < j:
                    parts.append(" " * (base + (1 if w - i < extra else 0)))
            line = "".join(parts)
        res.append(line)
        i = j + 1
    return res


if __name__ == "__main__":
    words = ["the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog"]
    for line in justify(words, 16):
        print(line)
