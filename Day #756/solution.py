# Day 756: Recover digits from an anagram of their English spellings.
# Use unique marker letters (z,w,u,x,g) then deduce the rest. Time: O(n), Space: O(1).
from collections import Counter


def recover_digits(s):
    cnt = Counter(s)
    d = [0] * 10
    d[0] = cnt['z']
    d[2] = cnt['w']
    d[4] = cnt['u']
    d[6] = cnt['x']
    d[8] = cnt['g']
    d[1] = cnt['o'] - d[0] - d[2] - d[4]
    d[3] = cnt['h'] - d[8]
    d[5] = cnt['f'] - d[4]
    d[7] = cnt['s'] - d[6]
    d[9] = cnt['i'] - d[5] - d[6] - d[8]
    return "".join(str(i) * d[i] for i in range(10))


if __name__ == "__main__":
    print(recover_digits("niesevehrtfeev"))  # 357
