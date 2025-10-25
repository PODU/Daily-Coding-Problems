# Day 489: Longest subarray with all distinct elements.
# Sliding window + last-seen map; move left past previous occurrence. Time O(n), Space O(n).
def longest_distinct_subarray(a):
    last = {}
    left = 0
    best = 0
    for right, val in enumerate(a):
        if val in last and last[val] >= left:
            left = last[val] + 1
        last[val] = right
        best = max(best, right - left + 1)
    return best


if __name__ == "__main__":
    print(longest_distinct_subarray([5, 1, 3, 5, 2, 3, 4, 1]))  # 5
