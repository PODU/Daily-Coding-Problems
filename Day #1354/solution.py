# Day 1354: Minimum jumps to reach end. Greedy: track current reach, farthest reach, jumps. O(N) time, O(1) space.

def min_jumps(a):
    n = len(a)
    if n <= 1:
        return 0
    jumps = 0
    cur_end = 0
    farthest = 0
    for i in range(n - 1):
        farthest = max(farthest, i + a[i])
        if i == cur_end:
            jumps += 1
            cur_end = farthest
            if cur_end >= n - 1:
                break
    return jumps


if __name__ == "__main__":
    a = [6, 2, 4, 0, 5, 1, 1, 4, 2, 9]
    print(min_jumps(a))
