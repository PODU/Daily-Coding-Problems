# Day 245: Minimum jumps to reach end: greedy window/BFS-level expansion. O(n) time, O(1) space.


def min_jumps(a):
    n = len(a)
    if n <= 1:
        return 0
    jumps = cur_end = farthest = 0
    for i in range(n - 1):
        farthest = max(farthest, i + a[i])
        if i == cur_end:
            jumps += 1
            cur_end = farthest
            if cur_end >= n - 1:
                break
    return jumps


def main():
    a = [6, 2, 4, 0, 5, 1, 1, 4, 2, 9]
    print(min_jumps(a))


if __name__ == "__main__":
    main()
