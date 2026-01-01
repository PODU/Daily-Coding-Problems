# Day 835: Shortest substring containing all chars in a set.
# Sliding-window min-window: expand right, contract left while valid. O(N) time, O(K) space.

def shortest_substring(s, charset):
    need = {c: 1 for c in charset}
    required = len(need)
    have = {c: 0 for c in charset}
    formed = 0
    left = 0
    best = None
    for right, ch in enumerate(s):
        if ch in need:
            have[ch] += 1
            if have[ch] == need[ch]:
                formed += 1
        while formed == required:
            if best is None or right - left + 1 < len(best):
                best = s[left:right + 1]
            lc = s[left]
            if lc in need:
                have[lc] -= 1
                if have[lc] < need[lc]:
                    formed -= 1
            left += 1
    return best


if __name__ == "__main__":
    res = shortest_substring("figehaeci", {'a', 'e', 'i'})
    print(res if res is not None else "null")
