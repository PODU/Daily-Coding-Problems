# Day 57: Greedy word wrap into lines of length <= k. None if any word > k.
# Time: O(n), Space: O(n).


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
    res = wrap("the quick brown fox jumps over the lazy dog", 10)
    print("null" if res is None else res)
