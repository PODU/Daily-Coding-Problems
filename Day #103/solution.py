# Day 103: Shortest window containing all chars of a set, via sliding window with
# a required-count and a "have all" tracker. O(n) time, O(set) space.
def min_window(s, chars):
    need = set(chars)
    if not need:
        return ""
    count = {}
    have = 0
    best = None
    left = 0
    for right, ch in enumerate(s):
        if ch in need:
            count[ch] = count.get(ch, 0) + 1
            if count[ch] == 1:
                have += 1
        while have == len(need):
            if best is None or right - left + 1 < len(best):
                best = s[left:right + 1]
            lc = s[left]
            if lc in need:
                count[lc] -= 1
                if count[lc] == 0:
                    have -= 1
            left += 1
    return best


if __name__ == "__main__":
    print(min_window("figehaeci", {'a', 'e', 'i'}))  # aeci
