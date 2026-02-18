# Day 1097: Smallest string by moving one of first k letters to the end.
# k==1 -> only rotations reachable -> min rotation; k>=2 -> any order -> sorted.
# Time: O(N^2) for k==1, O(N log N) for k>=2. Space: O(N).
def smallest(s, k):
    if k >= 2:
        return "".join(sorted(s))
    best = s
    cur = s
    for _ in range(len(s)):
        cur = cur[1:] + cur[0]
        if cur < best:
            best = cur
    return best


if __name__ == "__main__":
    print(smallest("daily", 1))  # ailyd
