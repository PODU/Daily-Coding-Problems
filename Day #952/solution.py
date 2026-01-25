# Day 952: decode an anagram of spelled-out digit words (zero..nine) -> sorted digits.
# Use unique marker letters to count each digit. Time O(n), Space O(1).
from collections import Counter


def decode(s):
    c = Counter(s)
    cnt = [0] * 10
    cnt[0] = c['z']
    cnt[2] = c['w']
    cnt[4] = c['u']
    cnt[6] = c['x']
    cnt[8] = c['g']
    cnt[3] = c['h'] - cnt[8]
    cnt[5] = c['f'] - cnt[4]
    cnt[7] = c['s'] - cnt[6]
    cnt[1] = c['o'] - cnt[0] - cnt[2] - cnt[4]
    cnt[9] = c['i'] - cnt[5] - cnt[6] - cnt[8]
    return "".join(str(d) * cnt[d] for d in range(10))


if __name__ == "__main__":
    print(decode("niesevehrtfeev"))  # 357
