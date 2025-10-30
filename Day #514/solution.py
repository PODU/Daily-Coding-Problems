# Day 514: Longest consecutive sequence: hash set, extend only from sequence starts. O(n) time/space.
def longest_consecutive(a):
    s = set(a)
    best = 0
    for x in s:
        if x - 1 in s:
            continue
        length, cur = 1, x
        while cur + 1 in s:
            cur += 1
            length += 1
        best = max(best, length)
    return best


if __name__ == "__main__":
    print(longest_consecutive([100, 4, 200, 1, 3, 2]))
