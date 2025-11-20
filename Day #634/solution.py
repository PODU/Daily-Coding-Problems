# Day 634: Largest rectangle in a histogram.
# Approach: monotonic increasing stack of bar indices; pop to settle areas.
# Time: O(N), Space: O(N).
def largest_rectangle(h):
    st = []
    best = 0
    n = len(h)
    for i in range(n + 1):
        cur = 0 if i == n else h[i]
        while st and h[st[-1]] >= cur:
            height = h[st.pop()]
            left = st[-1] if st else -1
            width = i - left - 1
            best = max(best, height * width)
        st.append(i)
    return best


if __name__ == "__main__":
    print(largest_rectangle([1, 3, 2, 5]))  # 6
