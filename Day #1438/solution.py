# Day 1438: Largest rectangle in a histogram.
# Approach: monotonic increasing stack of bar indices; pop to compute areas.
# Time: O(n), Space: O(n).


def largest_rectangle(heights):
    st = []  # indices with increasing heights
    best = 0
    n = len(heights)
    for i in range(n + 1):
        h = 0 if i == n else heights[i]
        while st and heights[st[-1]] >= h:
            top = st.pop()
            width = i if not st else i - st[-1] - 1
            best = max(best, heights[top] * width)
        st.append(i)
    return best


if __name__ == "__main__":
    print(largest_rectangle([1, 3, 2, 5]))  # 6
