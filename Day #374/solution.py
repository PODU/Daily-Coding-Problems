# Day 374: Lowest index i with arr[i]==i in a sorted, distinct-int array.
# f(i)=arr[i]-i is non-decreasing, so binary-search the leftmost i with
# f(i)>=0 and check equality. Time O(log n), Space O(1).


def fixed_point(arr):
    lo, hi, ans = 0, len(arr) - 1, -1
    while lo <= hi:
        mid = (lo + hi) // 2
        if arr[mid] >= mid:
            ans = mid
            hi = mid - 1
        else:
            lo = mid + 1
    if ans != -1 and arr[ans] == ans:
        return ans
    return None


if __name__ == "__main__":
    print(fixed_point([-5, -3, 2, 3]))  # 2
