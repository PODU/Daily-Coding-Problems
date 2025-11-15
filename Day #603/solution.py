# Day 603: Final resting state of pushed dominoes.
# Approach: two force passes (R rightward, L leftward), compare. Time O(n), Space O(n).


def push_dominoes(s):
    n = len(s)
    fr = [0] * n  # force pushing right
    fl = [0] * n  # force pushing left
    f = 0
    for i in range(n):
        if s[i] == 'R':
            f = n
        elif s[i] == 'L':
            f = 0
        else:
            f = max(f - 1, 0)
        fr[i] = f
    f = 0
    for i in range(n - 1, -1, -1):
        if s[i] == 'L':
            f = n
        elif s[i] == 'R':
            f = 0
        else:
            f = max(f - 1, 0)
        fl[i] = f
    res = []
    for i in range(n):
        if fr[i] > fl[i]:
            res.append('R')
        elif fr[i] < fl[i]:
            res.append('L')
        else:
            res.append('.')
    return ''.join(res)


def main():
    print(push_dominoes(".L.R....L"))  # LL.RRRLLL
    print(push_dominoes("..R...L.L"))  # ..RR.LLLL


if __name__ == '__main__':
    main()
