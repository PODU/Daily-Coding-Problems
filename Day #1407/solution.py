# Day 1407: Single pass from the west end (array right), tracking the tallest seen so far;
# a building has a view iff it is taller than everything to its west.
# Time O(n), Space O(1).

def count_sunset_views(h):
    count, max_w = 0, float('-inf')
    for x in reversed(h):
        if x > max_w:
            count += 1
            max_w = x
    return count


if __name__ == "__main__":
    print(count_sunset_views([3, 7, 8, 3, 6, 1]))  # 3
