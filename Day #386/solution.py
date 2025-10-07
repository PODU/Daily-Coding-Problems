# Day 386: Sort characters by descending frequency (ties: first-occurrence order).
# Time: O(n log d), Space: O(n).

def frequency_sort(s):
    cnt, first = {}, {}
    for i, c in enumerate(s):
        cnt[c] = cnt.get(c, 0) + 1
        first.setdefault(c, i)
    chars = sorted(cnt, key=lambda c: (-cnt[c], first[c]))
    return "".join(c * cnt[c] for c in chars)


if __name__ == "__main__":
    print(frequency_sort("tweet"))
