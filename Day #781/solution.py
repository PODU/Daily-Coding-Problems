# Day 781: Largest rectangle in a histogram.
# Monotonic increasing stack of bar indices; pop when a lower bar arrives. O(n) time, O(n) space.

def largest_rectangle(h):
    st = []  # indices of increasing heights
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
    heights = [1, 3, 2, 5]
    print(largest_rectangle(heights))
