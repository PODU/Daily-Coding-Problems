# Day 854: greedy word wrap - pack max words per line of length <= k; None if any word > k.
# Single pass over words. O(total characters).

def wrap(s, k):
    out = []
    line = ""
    for word in s.split(" "):
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
