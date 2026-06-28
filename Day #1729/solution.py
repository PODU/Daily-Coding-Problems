# Day 1729: Count buildings with a sunset (west) view.
# Single right-to-left pass tracking max height seen to the west; a building is
# visible iff strictly taller than all to its west. Time: O(n). Space: O(1).


def count_sunset_views(heights):
    count = 0
    max_west = 0
    # Scan from the west end (rightmost index) toward the east.
    for h in reversed(heights):
        if h > max_west:
            count += 1
            max_west = h
    return count


def main():
    heights = [3, 7, 8, 3, 6, 1]  # east -> west
    print(count_sunset_views(heights))  # 1, 6, 8 visible => 3


if __name__ == "__main__":
    main()
