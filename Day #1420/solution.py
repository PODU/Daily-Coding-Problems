# Day 1420: wrap words into lines of length <= k, greedily packing max words/line.
# Approach: greedy single pass over words. O(n) time, O(n) space. None if a word > k.


def wrap(s, k):
    out = []
    line = ""
    for word in s.split(" "):
        if len(word) > k:
            return None  # impossible
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
