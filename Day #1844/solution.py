# Day 1844: Largest rectangle in a histogram via a monotonic increasing stack.
# Time O(N), Space O(N).


def largest_rectangle(heights):
    h = heights + [0]  # sentinel flushes the stack
    stack = []         # indices of increasing bars
    best = 0
    for i, cur in enumerate(h):
        while stack and h[stack[-1]] >= cur:
            height = h[stack.pop()]
            left = stack[-1] if stack else -1
            width = i - left - 1
            best = max(best, height * width)
        stack.append(i)
    return best


def main():
    print(largest_rectangle([1, 3, 2, 5]))  # 6


if __name__ == "__main__":
    main()
