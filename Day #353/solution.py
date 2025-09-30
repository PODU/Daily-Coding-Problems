# Day 353: Largest rectangle in histogram via monotonic increasing stack. Time O(N), Space O(N).

def largest_rectangle(heights):
    stack = []  # indices of increasing bars
    best = 0
    for i in range(len(heights) + 1):
        h = heights[i] if i < len(heights) else 0
        while stack and heights[stack[-1]] >= h:
            height = heights[stack.pop()]
            width = i if not stack else i - stack[-1] - 1
            best = max(best, height * width)
        stack.append(i)
    return best


def main():
    print(largest_rectangle([1, 3, 2, 5]))


if __name__ == "__main__":
    main()
