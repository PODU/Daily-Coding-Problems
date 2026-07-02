# Day 1748: Recover digits from an anagram of concatenated number words (zero-nine).
# Approach: unique-letter signatures (z,w,u,x,g first; then derive odd digits). O(n) time, O(1) space.
from collections import Counter


def recover(s):
    c = Counter(s)
    cnt = [0] * 10
    cnt[0] = c['z']                          # zero
    cnt[2] = c['w']                          # two
    cnt[4] = c['u']                          # four
    cnt[6] = c['x']                          # six
    cnt[8] = c['g']                          # eight
    cnt[3] = c['h'] - cnt[8]                 # three (h also in eight)
    cnt[5] = c['f'] - cnt[4]                 # five  (f also in four)
    cnt[7] = c['s'] - cnt[6]                 # seven (s also in six)
    cnt[1] = c['o'] - cnt[0] - cnt[2] - cnt[4]   # one (o in zero,two,four)
    cnt[9] = c['i'] - cnt[5] - cnt[6] - cnt[8]   # nine (i in five,six,eight)
    return "".join(str(d) * cnt[d] for d in range(10))


def main():
    print(recover("niesevehrtfeev"))  # 357


if __name__ == "__main__":
    main()
