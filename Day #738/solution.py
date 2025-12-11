# Day 738: Greedy line wrapping: fit max words per line within width k.
# Return None if any single word exceeds k.
# Time: O(n), Space: O(n).

def wrap(s, k):
    words = s.split(" ")
    out, line = [], ""
    for word in words:
        if len(word) > k:
            return None
        if not line:
            line = word
        elif len(line) + 1 + len(word) <= k:
            line += " " + word
        else:
            out.append(line)
            line = word
    if line:
        out.append(line)
    return out


if __name__ == "__main__":
    print(wrap("the quick brown fox jumps over the lazy dog", 10))
    # ['the quick', 'brown fox', 'jumps over', 'the lazy', 'dog']
