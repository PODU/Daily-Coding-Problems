# Day 1797: Word wrap greedily: pack max words per line <= k, return None if any word > k. O(total length) time.
def word_wrap(s, k):
    lines, cur = [], ""
    for w in s.split(" "):
        if len(w) > k:
            return None
        if not cur:
            cur = w
        elif len(cur) + 1 + len(w) <= k:
            cur += " " + w
        else:
            lines.append(cur)
            cur = w
    if cur:
        lines.append(cur)
    return lines


if __name__ == "__main__":
    res = word_wrap("the quick brown fox jumps over the lazy dog", 10)
    if res is None:
        print("None")
    else:
        print("[" + ", ".join('"' + ln + '"' for ln in res) + "]")
