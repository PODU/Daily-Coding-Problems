# Day 1659: Smallest unsorted window. Scan L->R tracking max for end, R->L tracking min for start. O(n) time, O(1) space.
def unsorted_window(a):
    n = len(a)
    end = start = -1
    mx = float("-inf")
    mn = float("inf")
    for i in range(n):
        mx = max(mx, a[i])
        if a[i] < mx:
            end = i
    for i in range(n - 1, -1, -1):
        mn = min(mn, a[i])
        if a[i] > mn:
            start = i
    return start, end

if __name__ == "__main__":
    s, e = unsorted_window([3, 7, 5, 6, 9])
    print(f"({s}, {e})")
