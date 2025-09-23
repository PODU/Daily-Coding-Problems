# Day 320: Smallest window containing all distinct chars: O(n) sliding window.
# Time O(n), Space O(alphabet).
from collections import defaultdict

def smallest_window(s):
    need = len(set(s))
    cnt = defaultdict(int)
    have = 0
    left = 0
    best = float("inf")
    for right, ch in enumerate(s):
        cnt[ch] += 1
        if cnt[ch] == 1:
            have += 1
        while have == need:
            best = min(best, right - left + 1)
            cnt[s[left]] -= 1
            if cnt[s[left]] == 0:
                have -= 1
            left += 1
    return best

if __name__ == "__main__":
    print(smallest_window("jiujitsu"))
