# Day 1617: Falling dominoes: two-pass force accumulation (R adds +, L adds -, decay between).
# Sign of net force decides L/R/.; equal force stays '.'. Time O(n), Space O(n).

def push_dominoes(d):
    n = len(d)
    force = [0] * n
    f = 0
    for i in range(n):
        if d[i] == 'R':
            f = n
        elif d[i] == 'L':
            f = 0
        else:
            f = max(f - 1, 0)
        force[i] += f
    f = 0
    for i in range(n - 1, -1, -1):
        if d[i] == 'L':
            f = n
        elif d[i] == 'R':
            f = 0
        else:
            f = max(f - 1, 0)
        force[i] -= f
    return ''.join('R' if x > 0 else 'L' if x < 0 else '.' for x in force)


if __name__ == "__main__":
    print(push_dominoes(".L.R....L"))
    print(push_dominoes("..R...L.L"))
