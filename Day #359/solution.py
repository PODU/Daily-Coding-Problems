# Day 359: Recover digits from scrambled letters: use unique marker letters (z,w,u,x,g) then derive the rest. O(N) time, O(1) space.
from collections import Counter

def recover(s):
    c = Counter(s)
    d = [0] * 10
    d[0] = c['z']
    d[2] = c['w']
    d[4] = c['u']
    d[6] = c['x']
    d[8] = c['g']
    d[3] = c['h'] - d[8]
    d[5] = c['f'] - d[4]
    d[7] = c['s'] - d[6]
    d[1] = c['o'] - d[0] - d[2] - d[4]
    d[9] = c['i'] - d[5] - d[6] - d[8]
    return ''.join(str(i) * d[i] for i in range(10))

if __name__ == "__main__":
    print(recover("niesevehrtfeev"))
