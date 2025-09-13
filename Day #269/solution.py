# Day 269: Push dominoes simulation via force/two-pointer scan.
# Left-to-right add +force from R, right-to-left add -force from L, sign decides. Time O(n), Space O(n).

def push_dominoes(s: str) -> str:
    n = len(s)
    force = [0] * n
    f = 0
    for i in range(n):                 # rightward push
        if s[i] == 'R':
            f = n
        elif s[i] == 'L':
            f = 0
        else:
            f = max(f - 1, 0)
        force[i] += f
    f = 0
    for i in range(n - 1, -1, -1):     # leftward push
        if s[i] == 'L':
            f = n
        elif s[i] == 'R':
            f = 0
        else:
            f = max(f - 1, 0)
        force[i] -= f
    return ''.join('R' if v > 0 else 'L' if v < 0 else '.' for v in force)


if __name__ == "__main__":
    print(push_dominoes(".L.R....L"))
    print(push_dominoes("..R...L.L"))
